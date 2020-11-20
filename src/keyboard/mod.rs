use self::{key_code::KeyCode, output_char::OutputChar};
use anyhow::*;

pub mod backend;
pub mod chord;
pub mod key_code;
pub mod output_char;

pub enum KeyEvent {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
}

pub trait Backend {
    /// read events from the backend, and execute the given lambda on each of these events.
    fn handle_events<F: FnMut(KeyEvent)>(&self, f: F) -> Result<()>;

    /// Write a keyevent to the backend
    fn send_key_event(&self, event: KeyEvent) -> Result<()>;

    /// Write a list of `OutputChar`s
    fn write_chars(&self, chars: &[OutputChar]) -> Result<()> {
        for c in chars {
            self.write_char(c)?;
        }
        Ok(())
    }

    /// Write a single `OutputChar`
    fn write_char(&self, output: &OutputChar) -> Result<()> {
        if output.is_upper() {
            self.send_key_event(KeyEvent::KeyDown(KeyCode::KEY_LEFTSHIFT))?;
        }
        self.press_key(output.key)?;
        if output.is_upper() {
            self.send_key_event(KeyEvent::KeyUp(KeyCode::KEY_LEFTSHIFT))?;
        }
        Ok(())
    }

    /// Hit a single key for s short moment
    fn press_key(&self, key: KeyCode) -> Result<()> {
        self.send_key_event(KeyEvent::KeyDown(key))?;
        self.send_key_event(KeyEvent::KeyUp(key))?;
        Ok(())
    }
}
