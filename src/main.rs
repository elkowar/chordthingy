use evdev_rs::{
    enums::EventCode, enums::EV_KEY, enums::EV_SYN, Device, InputEvent, TimeVal, UInputDevice,
};
use itertools::Itertools;

use std::{collections::HashMap, fs::File};

use anyhow::*;

fn main() -> Result<()> {
    let device_path = std::env::args()
        .nth(1)
        .unwrap_or("/dev/input/event21".to_owned());

    let device_file = File::open(device_path)?;

    let mut device = Device::new().context("Error getting device")?;
    device.set_fd(device_file)?;
    let input_device = UInputDevice::create_from_device(&device)?;
    let input_device = InputDevice::new(input_device);

    let mut state = HashMap::new();

    loop {
        let event = device.next_event(evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING);
        match event {
            Ok((_, event)) => match (event.value, event.event_code) {
                (0, EventCode::EV_KEY(code)) => {
                    state.insert(code, false);

                    if state.values().all(|x| !*x) {
                        handle_keypress(
                            &input_device,
                            Chord::from_ev_keys(&state.drain().map(|(k, _)| k).collect::<Vec<_>>()),
                        )?;
                    }
                }
                (_, EventCode::EV_KEY(code)) => {
                    state.insert(code, true);
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("{:#?}", e);
            }
        }
    }
}

struct InputDevice(UInputDevice);

impl InputDevice {
    fn new(x: UInputDevice) -> Self {
        InputDevice(x)
    }

    fn write_output_char(&self, output_char: &OutputChar) -> Result<()> {
        if output_char.is_upper {
            self.send_key(&EV_KEY::KEY_LEFTSHIFT, 1)?;
        }
        self.press_key(&output_char.key)?;
        if output_char.is_upper {
            self.send_key(&EV_KEY::KEY_LEFTSHIFT, 0)?;
        }
        Ok(())
    }

    fn press_key(&self, key: &EV_KEY) -> Result<()> {
        self.send_key(key, 1)?;
        self.send_key(key, 0)?;
        Ok(())
    }

    fn send_key(&self, key: &EV_KEY, state: i32) -> Result<()> {
        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        self.0
            .write_event(&InputEvent::new(
                &TimeVal::new(now_millis, 0),
                &EventCode::EV_KEY(key.clone()),
                state,
            ))
            .unwrap();
        self.0.write_event(&InputEvent::new(
            &TimeVal::new(now_millis, 0),
            &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            0,
        ))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Chord(Vec<String>);

impl Chord {
    fn new<S: AsRef<str>>(s: S) -> Self {
        Chord(s.as_ref().chars().map(|x| x.to_string()).sorted().collect())
    }
}

impl Chord {
    fn from_ev_keys(keys: &[EV_KEY]) -> Self {
        let parts = keys
            .iter()
            .map(|key| match key {
                EV_KEY::KEY_A => "a",
                EV_KEY::KEY_B => "b",
                EV_KEY::KEY_C => "c",
                EV_KEY::KEY_D => "d",
                EV_KEY::KEY_E => "e",
                EV_KEY::KEY_F => "f",
                EV_KEY::KEY_G => "g",
                EV_KEY::KEY_H => "h",
                EV_KEY::KEY_I => "i",
                EV_KEY::KEY_J => "j",
                EV_KEY::KEY_K => "k",
                EV_KEY::KEY_L => "l",
                EV_KEY::KEY_M => "m",
                EV_KEY::KEY_N => "n",
                EV_KEY::KEY_O => "o",
                EV_KEY::KEY_P => "p",
                EV_KEY::KEY_Q => "q",
                EV_KEY::KEY_R => "r",
                EV_KEY::KEY_S => "s",
                EV_KEY::KEY_T => "t",
                EV_KEY::KEY_U => "u",
                EV_KEY::KEY_V => "v",
                EV_KEY::KEY_W => "w",
                EV_KEY::KEY_X => "x",
                EV_KEY::KEY_Y => "y",
                EV_KEY::KEY_Z => "z",
                EV_KEY::KEY_SPACE => " ",
                _ => "<???>",
            })
            .map(|x| x.to_owned())
            .sorted()
            .collect_vec();
        Chord(parts)
    }
}

fn handle_keypress(device: &InputDevice, chord: Chord) -> Result<()> {
    let data: HashMap<Chord, _> = maplit::hashmap! {
        Chord::new("hst") => "something",
        Chord::new("fuk") => "fuck",
        Chord::new("gob") => "golang is fucking disgusting",
    };
    let data: HashMap<Chord, _> = data
        .into_iter()
        .map(|(k, v)| Ok((k, output_chars_from_string(v)?)))
        .collect::<Result<_>>()?;

    let result: Option<&Vec<OutputChar>> = data.get(&chord);

    if let Some(result) = result {
        for _ in 0..chord.0.len() {
            device.write_output_char(&OutputChar::from(EV_KEY::KEY_BACKSPACE))?;
        }
        for key in result {
            device.write_output_char(key)?;
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct OutputChar {
    key: EV_KEY,
    is_upper: bool,
}

impl From<EV_KEY> for OutputChar {
    fn from(key: EV_KEY) -> Self {
        OutputChar {
            key,
            is_upper: false,
        }
    }
}

impl OutputChar {
    fn from_char(c: char) -> Result<OutputChar> {
        OutputChar::from_str(&c.to_string())
    }

    fn from_str(s: &str) -> Result<OutputChar> {
        let key = match s.to_ascii_lowercase().as_str() {
            "a" => EV_KEY::KEY_A,
            "b" => EV_KEY::KEY_B,
            "c" => EV_KEY::KEY_C,
            "d" => EV_KEY::KEY_D,
            "e" => EV_KEY::KEY_E,
            "f" => EV_KEY::KEY_F,
            "g" => EV_KEY::KEY_G,
            "h" => EV_KEY::KEY_H,
            "i" => EV_KEY::KEY_I,
            "j" => EV_KEY::KEY_J,
            "k" => EV_KEY::KEY_K,
            "l" => EV_KEY::KEY_L,
            "m" => EV_KEY::KEY_M,
            "n" => EV_KEY::KEY_N,
            "o" => EV_KEY::KEY_O,
            "p" => EV_KEY::KEY_P,
            "q" => EV_KEY::KEY_Q,
            "r" => EV_KEY::KEY_R,
            "s" => EV_KEY::KEY_S,
            "t" => EV_KEY::KEY_T,
            "u" => EV_KEY::KEY_U,
            "v" => EV_KEY::KEY_V,
            "w" => EV_KEY::KEY_W,
            "x" => EV_KEY::KEY_X,
            "y" => EV_KEY::KEY_Y,
            "z" => EV_KEY::KEY_Z,
            " " => EV_KEY::KEY_SPACE,
            c => bail!("Unrecognized character: {}", c),
        };

        Ok(OutputChar {
            key,
            is_upper: s.len() == 1 && s.chars().next().unwrap().is_uppercase(),
        })
    }
}

fn output_chars_from_string(s: &str) -> Result<Vec<OutputChar>> {
    s.chars().map(OutputChar::from_char).collect::<Result<_>>()
}
