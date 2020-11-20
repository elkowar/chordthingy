use anyhow::*;
use evdev_rs::{
    enums::EventCode, enums::EV_KEY, enums::EV_SYN, Device, InputEvent, TimeVal, UInputDevice,
};

use crate::keyboard::{key_code::KeyCode, Backend, KeyEvent};

pub struct EvDevBackend {
    input_device: UInputDevice,
    device: Device,
}

impl EvDevBackend {
    pub fn new(input_device: UInputDevice, device: Device) -> Self {
        EvDevBackend {
            input_device,
            device,
        }
    }
}

impl Backend for EvDevBackend {
    fn handle_events<F: FnMut(KeyEvent)>(&self, mut f: F) -> Result<()> {
        loop {
            let event = self
                .device
                .next_event(evdev_rs::ReadFlag::NORMAL | evdev_rs::ReadFlag::BLOCKING);

            match event {
                Ok((_, event)) => match (event.value, event.event_code) {
                    (0, EventCode::EV_KEY(code)) => f(KeyEvent::KeyUp(code.into())),
                    (_, EventCode::EV_KEY(code)) => f(KeyEvent::KeyDown(code.into())),
                    _ => {}
                },

                Err(err) => eprintln!("Error while handling key event: {:#?}", err),
            }
        }
    }

    fn send_key_event(&self, event: KeyEvent) -> Result<()> {
        let (key_code, state) = match event {
            KeyEvent::KeyUp(code) => (code, 0),
            KeyEvent::KeyDown(code) => (code, 1),
        };

        let now_millis = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;

        self.input_device
            .write_event(&InputEvent::new(
                &TimeVal::new(now_millis, 0),
                &EventCode::EV_KEY(key_code.into()),
                state,
            ))
            .unwrap();
        self.input_device.write_event(&InputEvent::new(
            &TimeVal::new(now_millis, 5),
            &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
            0,
        ))?;
        Ok(())
    }
}

impl Into<EV_KEY> for KeyCode {
    fn into(self) -> EV_KEY {
        match self {
            KeyCode::KEY_0 => EV_KEY::KEY_0,
            KeyCode::KEY_1 => EV_KEY::KEY_1,
            KeyCode::KEY_2 => EV_KEY::KEY_2,
            KeyCode::KEY_3 => EV_KEY::KEY_3,
            KeyCode::KEY_4 => EV_KEY::KEY_4,
            KeyCode::KEY_5 => EV_KEY::KEY_5,
            KeyCode::KEY_6 => EV_KEY::KEY_6,
            KeyCode::KEY_7 => EV_KEY::KEY_7,
            KeyCode::KEY_8 => EV_KEY::KEY_8,
            KeyCode::KEY_9 => EV_KEY::KEY_9,
            KeyCode::KEY_A => EV_KEY::KEY_A,
            KeyCode::KEY_B => EV_KEY::KEY_B,
            KeyCode::KEY_C => EV_KEY::KEY_C,
            KeyCode::KEY_D => EV_KEY::KEY_D,
            KeyCode::KEY_E => EV_KEY::KEY_E,
            KeyCode::KEY_F => EV_KEY::KEY_F,
            KeyCode::KEY_G => EV_KEY::KEY_G,
            KeyCode::KEY_H => EV_KEY::KEY_H,
            KeyCode::KEY_I => EV_KEY::KEY_I,
            KeyCode::KEY_J => EV_KEY::KEY_J,
            KeyCode::KEY_K => EV_KEY::KEY_K,
            KeyCode::KEY_L => EV_KEY::KEY_L,
            KeyCode::KEY_M => EV_KEY::KEY_M,
            KeyCode::KEY_N => EV_KEY::KEY_N,
            KeyCode::KEY_O => EV_KEY::KEY_O,
            KeyCode::KEY_P => EV_KEY::KEY_P,
            KeyCode::KEY_Q => EV_KEY::KEY_Q,
            KeyCode::KEY_R => EV_KEY::KEY_R,
            KeyCode::KEY_S => EV_KEY::KEY_S,
            KeyCode::KEY_T => EV_KEY::KEY_T,
            KeyCode::KEY_U => EV_KEY::KEY_U,
            KeyCode::KEY_V => EV_KEY::KEY_V,
            KeyCode::KEY_W => EV_KEY::KEY_W,
            KeyCode::KEY_X => EV_KEY::KEY_X,
            KeyCode::KEY_Y => EV_KEY::KEY_Y,
            KeyCode::KEY_Z => EV_KEY::KEY_Z,
            KeyCode::KEY_SPACE => EV_KEY::KEY_SPACE,
            KeyCode::KEY_F1 => EV_KEY::KEY_F1,
            KeyCode::KEY_F2 => EV_KEY::KEY_F2,
            KeyCode::KEY_F3 => EV_KEY::KEY_F3,
            KeyCode::KEY_F4 => EV_KEY::KEY_F4,
            KeyCode::KEY_F5 => EV_KEY::KEY_F5,
            KeyCode::KEY_F6 => EV_KEY::KEY_F6,
            KeyCode::KEY_F7 => EV_KEY::KEY_F7,
            KeyCode::KEY_F8 => EV_KEY::KEY_F8,
            KeyCode::KEY_F9 => EV_KEY::KEY_F9,
            KeyCode::KEY_FN => EV_KEY::KEY_FN,
            KeyCode::KEY_UP => EV_KEY::KEY_UP,
            KeyCode::KEY_DOT => EV_KEY::KEY_DOT,
            KeyCode::KEY_END => EV_KEY::KEY_END,
            KeyCode::KEY_ESC => EV_KEY::KEY_ESC,
            KeyCode::KEY_F10 => EV_KEY::KEY_F10,
            KeyCode::KEY_F11 => EV_KEY::KEY_F11,
            KeyCode::KEY_F12 => EV_KEY::KEY_F12,
            KeyCode::KEY_F13 => EV_KEY::KEY_F13,
            KeyCode::KEY_F14 => EV_KEY::KEY_F14,
            KeyCode::KEY_F15 => EV_KEY::KEY_F15,
            KeyCode::KEY_F16 => EV_KEY::KEY_F16,
            KeyCode::KEY_F17 => EV_KEY::KEY_F17,
            KeyCode::KEY_F18 => EV_KEY::KEY_F18,
            KeyCode::KEY_F19 => EV_KEY::KEY_F19,
            KeyCode::KEY_F20 => EV_KEY::KEY_F20,
            KeyCode::KEY_F21 => EV_KEY::KEY_F21,
            KeyCode::KEY_F22 => EV_KEY::KEY_F22,
            KeyCode::KEY_F23 => EV_KEY::KEY_F23,
            KeyCode::KEY_F24 => EV_KEY::KEY_F24,
            KeyCode::KEY_TAB => EV_KEY::KEY_TAB,
            KeyCode::KEY_BACK => EV_KEY::KEY_BACK,
            KeyCode::KEY_DOWN => EV_KEY::KEY_DOWN,
            KeyCode::KEY_HOME => EV_KEY::KEY_HOME,
            KeyCode::KEY_LEFT => EV_KEY::KEY_LEFT,
            KeyCode::KEY_MODE => EV_KEY::KEY_MODE,
            KeyCode::KEY_MOVE => EV_KEY::KEY_MOVE,
            KeyCode::KEY_ANGLE => EV_KEY::KEY_ANGLE,
            KeyCode::KEY_COMMA => EV_KEY::KEY_COMMA,
            KeyCode::KEY_ENTER => EV_KEY::KEY_ENTER,
            KeyCode::KEY_EQUAL => EV_KEY::KEY_EQUAL,
            KeyCode::KEY_GRAVE => EV_KEY::KEY_GRAVE,
            KeyCode::KEY_MINUS => EV_KEY::KEY_MINUS,
            KeyCode::KEY_RIGHT => EV_KEY::KEY_RIGHT,
            KeyCode::KEY_SLASH => EV_KEY::KEY_SLASH,
            KeyCode::KEY_DELETE => EV_KEY::KEY_DELETE,
            KeyCode::KEY_DOLLAR => EV_KEY::KEY_DOLLAR,
            KeyCode::KEY_INSERT => EV_KEY::KEY_INSERT,
            KeyCode::KEY_PAGEUP => EV_KEY::KEY_PAGEUP,
            KeyCode::KEY_LEFTALT => EV_KEY::KEY_LEFTALT,
            KeyCode::KEY_LEFT_UP => EV_KEY::KEY_LEFT_UP,
            KeyCode::KEY_CAPSLOCK => EV_KEY::KEY_CAPSLOCK,
            KeyCode::KEY_LEFTCTRL => EV_KEY::KEY_LEFTCTRL,
            KeyCode::KEY_LEFTMETA => EV_KEY::KEY_LEFTMETA,
            KeyCode::KEY_PAGEDOWN => EV_KEY::KEY_PAGEDOWN,
            KeyCode::KEY_RIGHTALT => EV_KEY::KEY_RIGHTALT,
            KeyCode::KEY_RIGHT_UP => EV_KEY::KEY_RIGHT_UP,
            KeyCode::KEY_BACKSLASH => EV_KEY::KEY_BACKSLASH,
            KeyCode::KEY_BACKSPACE => EV_KEY::KEY_BACKSPACE,
            KeyCode::KEY_LEFTBRACE => EV_KEY::KEY_LEFTBRACE,
            KeyCode::KEY_LEFT_DOWN => EV_KEY::KEY_LEFT_DOWN,
            KeyCode::KEY_LEFTSHIFT => EV_KEY::KEY_LEFTSHIFT,
            KeyCode::KEY_RIGHTCTRL => EV_KEY::KEY_RIGHTCTRL,
            KeyCode::KEY_RIGHTMETA => EV_KEY::KEY_RIGHTMETA,
            KeyCode::KEY_SEMICOLON => EV_KEY::KEY_SEMICOLON,
            KeyCode::KEY_APOSTROPHE => EV_KEY::KEY_APOSTROPHE,
            KeyCode::KEY_RIGHTBRACE => EV_KEY::KEY_RIGHTBRACE,
            KeyCode::KEY_RIGHT_DOWN => EV_KEY::KEY_RIGHT_DOWN,
            KeyCode::KEY_RIGHTSHIFT => EV_KEY::KEY_RIGHTSHIFT,
            _ => EV_KEY::KEY_UNKNOWN,
        }
    }
}

impl From<EV_KEY> for KeyCode {
    fn from(k: EV_KEY) -> Self {
        match k {
            EV_KEY::KEY_0 => KeyCode::KEY_0,
            EV_KEY::KEY_1 => KeyCode::KEY_1,
            EV_KEY::KEY_2 => KeyCode::KEY_2,
            EV_KEY::KEY_3 => KeyCode::KEY_3,
            EV_KEY::KEY_4 => KeyCode::KEY_4,
            EV_KEY::KEY_5 => KeyCode::KEY_5,
            EV_KEY::KEY_6 => KeyCode::KEY_6,
            EV_KEY::KEY_7 => KeyCode::KEY_7,
            EV_KEY::KEY_8 => KeyCode::KEY_8,
            EV_KEY::KEY_9 => KeyCode::KEY_9,
            EV_KEY::KEY_A => KeyCode::KEY_A,
            EV_KEY::KEY_B => KeyCode::KEY_B,
            EV_KEY::KEY_C => KeyCode::KEY_C,
            EV_KEY::KEY_D => KeyCode::KEY_D,
            EV_KEY::KEY_E => KeyCode::KEY_E,
            EV_KEY::KEY_F => KeyCode::KEY_F,
            EV_KEY::KEY_G => KeyCode::KEY_G,
            EV_KEY::KEY_H => KeyCode::KEY_H,
            EV_KEY::KEY_I => KeyCode::KEY_I,
            EV_KEY::KEY_J => KeyCode::KEY_J,
            EV_KEY::KEY_K => KeyCode::KEY_K,
            EV_KEY::KEY_L => KeyCode::KEY_L,
            EV_KEY::KEY_M => KeyCode::KEY_M,
            EV_KEY::KEY_N => KeyCode::KEY_N,
            EV_KEY::KEY_O => KeyCode::KEY_O,
            EV_KEY::KEY_P => KeyCode::KEY_P,
            EV_KEY::KEY_Q => KeyCode::KEY_Q,
            EV_KEY::KEY_R => KeyCode::KEY_R,
            EV_KEY::KEY_S => KeyCode::KEY_S,
            EV_KEY::KEY_T => KeyCode::KEY_T,
            EV_KEY::KEY_U => KeyCode::KEY_U,
            EV_KEY::KEY_V => KeyCode::KEY_V,
            EV_KEY::KEY_W => KeyCode::KEY_W,
            EV_KEY::KEY_X => KeyCode::KEY_X,
            EV_KEY::KEY_Y => KeyCode::KEY_Y,
            EV_KEY::KEY_Z => KeyCode::KEY_Z,
            EV_KEY::KEY_SPACE => KeyCode::KEY_SPACE,
            EV_KEY::KEY_F1 => KeyCode::KEY_F1,
            EV_KEY::KEY_F2 => KeyCode::KEY_F2,
            EV_KEY::KEY_F3 => KeyCode::KEY_F3,
            EV_KEY::KEY_F4 => KeyCode::KEY_F4,
            EV_KEY::KEY_F5 => KeyCode::KEY_F5,
            EV_KEY::KEY_F6 => KeyCode::KEY_F6,
            EV_KEY::KEY_F7 => KeyCode::KEY_F7,
            EV_KEY::KEY_F8 => KeyCode::KEY_F8,
            EV_KEY::KEY_F9 => KeyCode::KEY_F9,
            EV_KEY::KEY_FN => KeyCode::KEY_FN,
            EV_KEY::KEY_UP => KeyCode::KEY_UP,
            EV_KEY::KEY_DOT => KeyCode::KEY_DOT,
            EV_KEY::KEY_END => KeyCode::KEY_END,
            EV_KEY::KEY_ESC => KeyCode::KEY_ESC,
            EV_KEY::KEY_F10 => KeyCode::KEY_F10,
            EV_KEY::KEY_F11 => KeyCode::KEY_F11,
            EV_KEY::KEY_F12 => KeyCode::KEY_F12,
            EV_KEY::KEY_F13 => KeyCode::KEY_F13,
            EV_KEY::KEY_F14 => KeyCode::KEY_F14,
            EV_KEY::KEY_F15 => KeyCode::KEY_F15,
            EV_KEY::KEY_F16 => KeyCode::KEY_F16,
            EV_KEY::KEY_F17 => KeyCode::KEY_F17,
            EV_KEY::KEY_F18 => KeyCode::KEY_F18,
            EV_KEY::KEY_F19 => KeyCode::KEY_F19,
            EV_KEY::KEY_F20 => KeyCode::KEY_F20,
            EV_KEY::KEY_F21 => KeyCode::KEY_F21,
            EV_KEY::KEY_F22 => KeyCode::KEY_F22,
            EV_KEY::KEY_F23 => KeyCode::KEY_F23,
            EV_KEY::KEY_F24 => KeyCode::KEY_F24,
            EV_KEY::KEY_TAB => KeyCode::KEY_TAB,
            EV_KEY::KEY_BACK => KeyCode::KEY_BACK,
            EV_KEY::KEY_DOWN => KeyCode::KEY_DOWN,
            EV_KEY::KEY_HOME => KeyCode::KEY_HOME,
            EV_KEY::KEY_LEFT => KeyCode::KEY_LEFT,
            EV_KEY::KEY_MODE => KeyCode::KEY_MODE,
            EV_KEY::KEY_MOVE => KeyCode::KEY_MOVE,
            EV_KEY::KEY_ANGLE => KeyCode::KEY_ANGLE,
            EV_KEY::KEY_COMMA => KeyCode::KEY_COMMA,
            EV_KEY::KEY_ENTER => KeyCode::KEY_ENTER,
            EV_KEY::KEY_EQUAL => KeyCode::KEY_EQUAL,
            EV_KEY::KEY_GRAVE => KeyCode::KEY_GRAVE,
            EV_KEY::KEY_MINUS => KeyCode::KEY_MINUS,
            EV_KEY::KEY_RIGHT => KeyCode::KEY_RIGHT,
            EV_KEY::KEY_SLASH => KeyCode::KEY_SLASH,
            EV_KEY::KEY_DELETE => KeyCode::KEY_DELETE,
            EV_KEY::KEY_DOLLAR => KeyCode::KEY_DOLLAR,
            EV_KEY::KEY_INSERT => KeyCode::KEY_INSERT,
            EV_KEY::KEY_PAGEUP => KeyCode::KEY_PAGEUP,
            EV_KEY::KEY_LEFTALT => KeyCode::KEY_LEFTALT,
            EV_KEY::KEY_LEFT_UP => KeyCode::KEY_LEFT_UP,
            EV_KEY::KEY_CAPSLOCK => KeyCode::KEY_CAPSLOCK,
            EV_KEY::KEY_LEFTCTRL => KeyCode::KEY_LEFTCTRL,
            EV_KEY::KEY_LEFTMETA => KeyCode::KEY_LEFTMETA,
            EV_KEY::KEY_PAGEDOWN => KeyCode::KEY_PAGEDOWN,
            EV_KEY::KEY_RIGHTALT => KeyCode::KEY_RIGHTALT,
            EV_KEY::KEY_RIGHT_UP => KeyCode::KEY_RIGHT_UP,
            EV_KEY::KEY_BACKSLASH => KeyCode::KEY_BACKSLASH,
            EV_KEY::KEY_BACKSPACE => KeyCode::KEY_BACKSPACE,
            EV_KEY::KEY_LEFTBRACE => KeyCode::KEY_LEFTBRACE,
            EV_KEY::KEY_LEFT_DOWN => KeyCode::KEY_LEFT_DOWN,
            EV_KEY::KEY_LEFTSHIFT => KeyCode::KEY_LEFTSHIFT,
            EV_KEY::KEY_RIGHTCTRL => KeyCode::KEY_RIGHTCTRL,
            EV_KEY::KEY_RIGHTMETA => KeyCode::KEY_RIGHTMETA,
            EV_KEY::KEY_SEMICOLON => KeyCode::KEY_SEMICOLON,
            EV_KEY::KEY_APOSTROPHE => KeyCode::KEY_APOSTROPHE,
            EV_KEY::KEY_RIGHTBRACE => KeyCode::KEY_RIGHTBRACE,
            EV_KEY::KEY_RIGHT_DOWN => KeyCode::KEY_RIGHT_DOWN,
            EV_KEY::KEY_RIGHTSHIFT => KeyCode::KEY_RIGHTSHIFT,
            _ => KeyCode::UNKNOWN,
        }
    }
}
