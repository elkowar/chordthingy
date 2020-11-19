use evdev_rs::{enums::EventCode, enums::EV_KEY, enums::EV_SYN, InputEvent, TimeVal, UInputDevice};
use itertools::Itertools;

use anyhow::*;

use super::OutputChar;

pub struct InputDevice(UInputDevice);

impl InputDevice {
    pub fn new(x: UInputDevice) -> Self {
        InputDevice(x)
    }

    pub fn get_raw_device(&self) -> &UInputDevice {
        &self.0
    }

    pub fn write_output_char(&self, output_char: &OutputChar) -> Result<()> {
        //dbg!(output_char);
        if output_char.is_upper() {
            self.send_key(&EV_KEY::KEY_LEFTSHIFT, 1)?;
        }
        self.press_key(&output_char.key)?;
        if output_char.is_upper() {
            self.send_key(&EV_KEY::KEY_LEFTSHIFT, 0)?;
        }
        Ok(())
    }

    pub fn press_key(&self, key: &EV_KEY) -> Result<()> {
        //dbg!(key);
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
            &TimeVal::new(now_millis, 5),
            &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            0,
        ))?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Chord(Vec<String>);

impl Chord {
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let pattern = regex::Regex::new("<.*?>|.").unwrap();
        Chord(
            pattern
                .find_iter(s.as_ref())
                .map(|part| part.as_str().to_owned())
                .sorted()
                .dedup()
                .collect(),
        )
    }

    pub fn len(&self) -> usize {
        self.0.iter().filter(|x| x.len() == 1).count()
    }
}

impl Chord {
    pub fn from_ev_keys(keys: &[EV_KEY]) -> Self {
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
                EV_KEY::KEY_BACKSPACE => "<BS>",
                _ => "<???>",
            })
            .map(|x| x.to_owned())
            .sorted()
            .dedup()
            .collect_vec();
        Chord(parts)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_new_chord() {
        assert_eq!(Chord::new("<BS>"), Chord(vec!["<BS>".to_owned()]));
        assert_eq!(Chord::new("a"), Chord(vec!["a".to_owned()]));
        assert_eq!(
            Chord::new("aa<BS>a"),
            Chord(vec!["<BS>".to_owned(), "a".to_owned()])
        );
    }
}
