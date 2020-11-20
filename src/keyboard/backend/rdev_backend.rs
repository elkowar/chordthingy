use rdev;

use crate::keyboard::{key_code::KeyCode, Backend, KeyEvent};

pub struct RDevBackend {}

impl Backend for RDevBackend {
    fn handle_events<F: FnMut(KeyEvent)>(&self, f: F) -> anyhow::Result<()> {
        fn callback(event: rdev::Event) {
            if matches!(event.event_type, rdev::EventType::KeyPress(_) | rdev::EventType::KeyRelease(_))
            {
                let key_code = event.into();
                f(key_code);
            }
        }
        rdev::listen(callback).unwrap();
        Ok(())
    }

    fn send_key_event(&self, event: KeyEvent) -> anyhow::Result<()> {
        todo!()
    }
}

impl Into<KeyCode> for rdev::Event {
    fn into(self) -> KeyCode {
        if let Some(key) = self.name.and_then(|n| n.parse().ok()) {
            key
        } else {
            let key = match self.event_type {
                rdev::EventType::KeyPress(key) => key,
                rdev::EventType::KeyRelease(key) => key,
                _ => {
                    eprintln!("WARN: Tried to turn non press or release rdev event into KeyCode");
                    return KeyCode::UNKNOWN;
                }
            };
            match key {
                rdev::Key::Alt => KeyCode::KEY_LEFTALT,
                rdev::Key::AltGr => KeyCode::KEY_RIGHTALT,
                rdev::Key::BackSlash => KeyCode::KEY_BACKSLASH,
                rdev::Key::Backspace => KeyCode::KEY_BACKSPACE,
                rdev::Key::CapsLock => KeyCode::KEY_CAPSLOCK,
                rdev::Key::Comma => KeyCode::KEY_COMMA,
                rdev::Key::ControlLeft => KeyCode::KEY_LEFTCTRL,
                rdev::Key::ControlRight => KeyCode::KEY_RIGHTCTRL,
                rdev::Key::Delete => KeyCode::KEY_DELETE,
                rdev::Key::Dot => KeyCode::KEY_DOT,
                rdev::Key::DownArrow => KeyCode::KEY_DOWN,
                rdev::Key::End => KeyCode::KEY_END,
                rdev::Key::Equal => KeyCode::KEY_EQUAL,
                rdev::Key::Escape => KeyCode::KEY_ESC,
                rdev::Key::F1 => KeyCode::KEY_F1,
                rdev::Key::F10 => KeyCode::KEY_F10,
                rdev::Key::F11 => KeyCode::KEY_F11,
                rdev::Key::F12 => KeyCode::KEY_F12,
                rdev::Key::F2 => KeyCode::KEY_F2,
                rdev::Key::F3 => KeyCode::KEY_F3,
                rdev::Key::F4 => KeyCode::KEY_F4,
                rdev::Key::F5 => KeyCode::KEY_F5,
                rdev::Key::F6 => KeyCode::KEY_F6,
                rdev::Key::F7 => KeyCode::KEY_F7,
                rdev::Key::F8 => KeyCode::KEY_F8,
                rdev::Key::F9 => KeyCode::KEY_F9,
                rdev::Key::Home => KeyCode::KEY_HOME,
                rdev::Key::Insert => KeyCode::KEY_INSERT,
                rdev::Key::LeftArrow => KeyCode::KEY_LEFT,
                rdev::Key::MetaLeft => KeyCode::KEY_LEFTMETA,
                rdev::Key::MetaRight => KeyCode::KEY_RIGHTMETA,
                rdev::Key::Minus => KeyCode::KEY_MINUS,
                rdev::Key::PageDown => KeyCode::KEY_PAGEDOWN,
                rdev::Key::PageUp => KeyCode::KEY_PAGEUP,
                rdev::Key::Quote => KeyCode::KEY_APOSTROPHE,
                rdev::Key::Return => KeyCode::KEY_ENTER,
                rdev::Key::RightArrow => KeyCode::KEY_RIGHT,
                rdev::Key::SemiColon => KeyCode::KEY_SEMICOLON,
                rdev::Key::ShiftLeft => KeyCode::KEY_LEFTSHIFT,
                rdev::Key::ShiftRight => KeyCode::KEY_RIGHTSHIFT,
                rdev::Key::Slash => KeyCode::KEY_SLASH,
                rdev::Key::Space => KeyCode::KEY_SPACE,
                rdev::Key::Tab => KeyCode::KEY_TAB,
                rdev::Key::UpArrow => KeyCode::KEY_UP,
                _ => KeyCode::UNKNOWN,
            }
        }
    }
}
