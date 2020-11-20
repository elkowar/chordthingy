use anyhow::*;

use crate::{
    keyboard::output_char::OutputChar,
    keyboard::{chord::Chord, key_code::KeyCode, Backend, KeyEvent},
    mappings::Mappings,
};

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

pub struct App<B: Backend> {
    backend: B,
    mappings: Mappings,
    //history: HistoryList<HistoryEntry>,
}

impl<B: Backend> App<B> {
    pub fn new(backend: B, mappings: Mappings) -> Result<Self> {
        Ok(App {
            mappings,
            backend,
            //history: HistoryList::new(50),
        })
    }

    pub fn run(&mut self) -> Result<()> {
        let mut state = KeyPressState::default();

        self.backend.handle_events(|event| match event {
            KeyEvent::KeyDown(code) => {
                if state.none_released() != Some(false) {
                    state.press(code);
                } else {
                    state.clear();
                }
            }
            KeyEvent::KeyUp(code) => {
                state.release(&code);
                if state.all_released() == Some(true) {
                    let result = self.handle_keypress(Chord::from_key_codes(state.clear()));
                    if let Err(err) = result {
                        eprintln!("Error handling keypress: {:#?}", err);
                    }
                }
            }
        })?;
        Ok(())
    }

    pub fn handle_keypress(&self, chord: Chord) -> Result<()> {
        println!("{:?}", chord);

        let result: Option<&Vec<OutputChar>> = self.mappings.lookup(&chord);

        if let Some(result) = result {
            for _ in 0..chord.len() {
                self.backend.press_key(KeyCode::KEY_BACKSPACE)?;
            }
            std::thread::sleep(std::time::Duration::from_nanos(10));
            self.backend.write_chars(result)?;
        }

        Ok(())
    }
}
