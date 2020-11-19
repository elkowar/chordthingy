use anyhow::*;
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
};

use super::Chord;
use crate::{
    history::*,
    input::{InputDevice, OutputChar},
    mappings::Mappings,
};
use anyhow::*;
use evdev_rs::enums::{EventCode, EV_KEY};

#[derive(Debug, Eq, PartialEq)]
pub struct KeyPressState<K: Eq> {
    pressed_keys: Vec<(K, bool)>,
}

impl<K: Eq> Default for KeyPressState<K> {
    fn default() -> Self {
        KeyPressState {
            pressed_keys: Vec::new(),
        }
    }
}

impl<K: Eq> KeyPressState<K> {
    pub fn press(&mut self, k: K) {
        self.pressed_keys.push((k, true));
    }
    pub fn release(&mut self, k: &K) {
        if let Some((_, ref mut is_pressed)) =
            self.pressed_keys.iter_mut().find(|(key, _)| key == k)
        {
            *is_pressed = false;
        }
    }
    pub fn clear(&mut self) -> Vec<K> {
        self.pressed_keys.drain(..).map(|(k, _)| k).collect()
    }
    pub fn none_released(&self) -> Option<bool> {
        if self.pressed_keys.is_empty() {
            None
        } else {
            Some(self.pressed_keys.iter().all(|(_, pressed)| *pressed))
        }
    }
    pub fn all_released(&self) -> Option<bool> {
        if self.pressed_keys.is_empty() {
            None
        } else {
            Some(self.pressed_keys.iter().all(|(_, pressed)| !pressed))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_stuff() {
        let mut state = KeyPressState::default();

        assert_eq!(None, state.all_released());
        assert_eq!(None, state.none_released());
        state.press('a');
        assert_eq!(Some(true), state.none_released());
        assert_eq!(Some(false), state.all_released());
        state.press('b');
        assert_eq!(Some(true), state.none_released());
        assert_eq!(Some(false), state.all_released());
        state.release(&'b');
        assert_eq!(Some(false), state.none_released());
        assert_eq!(Some(false), state.all_released());
        state.release(&'a');
        assert_eq!(Some(false), state.none_released());
        assert_eq!(Some(true), state.all_released());
    }
}

pub struct App {
    device: evdev_rs::Device,
    input_device: InputDevice,
    mappings: Mappings,
    //history: HistoryList<HistoryEntry>,
}

impl App {
    pub fn new(mappings: Mappings) -> Result<Self> {
        let device_path = std::env::args()
            .nth(1)
            .unwrap_or("/dev/input/event10".to_owned());

        let device_file = std::fs::File::open(device_path)?;

        let mut device = evdev_rs::Device::new().context("Error getting device")?;
        device.set_fd(device_file)?;
        let input_device = evdev_rs::UInputDevice::create_from_device(&device)?;
        let input_device = InputDevice::new(input_device);

        Ok(App {
            device,
            input_device,
            mappings,
            //history: HistoryList::new(50),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut state = KeyPressState::default();
        loop {
            let event = self
                .device
                .next_event(evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING);

            match event {
                Ok((_, event)) => match (event.value, event.event_code) {
                    (0, EventCode::EV_KEY(code)) => {
                        state.release(&code);
                        if state.all_released() == Some(true) {
                            self.handle_keypress(Chord::from_ev_keys(&state.clear()))?;
                        }
                    }
                    (_, EventCode::EV_KEY(code)) => {
                        if state.none_released() != Some(false) {
                            state.press(code);
                        } else {
                            state.clear();
                        }
                    }
                    _ => {}
                },
                Err(e) => {
                    eprintln!("{:#?}", e);
                }
            }
        }
    }

    pub fn handle_keypress(&self, chord: Chord) -> Result<()> {
        println!("{:?}", chord);

        let result: Option<&Vec<OutputChar>> = self.mappings.lookup(&chord);

        if let Some(result) = result {
            for _ in 0..chord.len() {
                std::thread::sleep(std::time::Duration::from_nanos(10));
                self.input_device
                    .write_output_char(&OutputChar::from(EV_KEY::KEY_BACKSPACE))?;
            }
            std::thread::sleep(std::time::Duration::from_nanos(10));
            for key in result {
                std::thread::sleep(std::time::Duration::from_nanos(10));
                self.input_device.write_output_char(key)?;
            }
        }

        Ok(())
    }
}
