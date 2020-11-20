use std::str::FromStr;

use anyhow::bail;

#[allow(non_camel_case_types)]
#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum KeyCode {
    KEY_0,
    KEY_1,
    KEY_2,
    KEY_3,
    KEY_4,
    KEY_5,
    KEY_6,
    KEY_7,
    KEY_8,
    KEY_9,
    KEY_A,
    KEY_B,
    KEY_C,
    KEY_D,
    KEY_E,
    KEY_F,
    KEY_G,
    KEY_H,
    KEY_I,
    KEY_J,
    KEY_K,
    KEY_L,
    KEY_M,
    KEY_N,
    KEY_O,
    KEY_P,
    KEY_Q,
    KEY_R,
    KEY_S,
    KEY_T,
    KEY_U,
    KEY_V,
    KEY_W,
    KEY_X,
    KEY_Y,
    KEY_Z,
    KEY_F1,
    KEY_F2,
    KEY_F3,
    KEY_F4,
    KEY_F5,
    KEY_F6,
    KEY_F7,
    KEY_F8,
    KEY_F9,
    KEY_FN,
    KEY_UP,
    KEY_DOT,
    KEY_END,
    KEY_ESC,
    KEY_F10,
    KEY_F11,
    KEY_F12,
    KEY_F13,
    KEY_F14,
    KEY_F15,
    KEY_F16,
    KEY_F17,
    KEY_F18,
    KEY_F19,
    KEY_F20,
    KEY_F21,
    KEY_F22,
    KEY_F23,
    KEY_F24,
    KEY_TAB,
    KEY_BACK,
    KEY_DOWN,
    KEY_HOME,
    KEY_LEFT,
    KEY_MODE,
    KEY_MOVE,
    KEY_ANGLE,
    KEY_COMMA,
    KEY_ENTER,
    KEY_EQUAL,
    KEY_GRAVE,
    KEY_MINUS,
    KEY_RIGHT,
    KEY_SLASH,
    KEY_DELETE,
    KEY_DOLLAR,
    KEY_INSERT,
    KEY_PAGEUP,
    KEY_LEFTALT,
    KEY_LEFT_UP,
    KEY_CAPSLOCK,
    KEY_LEFTCTRL,
    KEY_LEFTMETA,
    KEY_PAGEDOWN,
    KEY_RIGHTALT,
    KEY_RIGHT_UP,
    KEY_BACKSLASH,
    KEY_BACKSPACE,
    KEY_LEFTBRACE,
    KEY_LEFT_DOWN,
    KEY_LEFTSHIFT,
    KEY_RIGHTCTRL,
    KEY_RIGHTMETA,
    KEY_SEMICOLON,
    KEY_APOSTROPHE,
    KEY_RIGHTBRACE,
    KEY_RIGHT_DOWN,
    KEY_RIGHTSHIFT,
    KEY_SPACE,
    UNKNOWN,
}

impl KeyCode {
    pub fn as_string(&self) -> &'static str {
        match self {
            KeyCode::KEY_0 => "0",
            KeyCode::KEY_1 => "1",
            KeyCode::KEY_2 => "2",
            KeyCode::KEY_3 => "3",
            KeyCode::KEY_4 => "4",
            KeyCode::KEY_5 => "5",
            KeyCode::KEY_6 => "6",
            KeyCode::KEY_7 => "7",
            KeyCode::KEY_8 => "8",
            KeyCode::KEY_9 => "9",
            KeyCode::KEY_A => "a",
            KeyCode::KEY_B => "b",
            KeyCode::KEY_C => "c",
            KeyCode::KEY_D => "d",
            KeyCode::KEY_E => "e",
            KeyCode::KEY_F => "f",
            KeyCode::KEY_G => "g",
            KeyCode::KEY_H => "h",
            KeyCode::KEY_I => "i",
            KeyCode::KEY_J => "j",
            KeyCode::KEY_K => "k",
            KeyCode::KEY_L => "l",
            KeyCode::KEY_M => "m",
            KeyCode::KEY_N => "n",
            KeyCode::KEY_O => "o",
            KeyCode::KEY_P => "p",
            KeyCode::KEY_Q => "q",
            KeyCode::KEY_R => "r",
            KeyCode::KEY_S => "s",
            KeyCode::KEY_T => "t",
            KeyCode::KEY_U => "u",
            KeyCode::KEY_V => "v",
            KeyCode::KEY_W => "w",
            KeyCode::KEY_X => "x",
            KeyCode::KEY_Y => "y",
            KeyCode::KEY_Z => "z",
            KeyCode::KEY_SPACE => " ",
            KeyCode::KEY_F1 => "<f1>",
            KeyCode::KEY_F2 => "<f2>",
            KeyCode::KEY_F3 => "<f3>",
            KeyCode::KEY_F4 => "<f4>",
            KeyCode::KEY_F5 => "<f5>",
            KeyCode::KEY_F6 => "<f6>",
            KeyCode::KEY_F7 => "<f7>",
            KeyCode::KEY_F8 => "<f8>",
            KeyCode::KEY_F9 => "<f9>",
            KeyCode::KEY_FN => "<fn>",
            KeyCode::KEY_UP => "<up>",
            KeyCode::KEY_DOT => "<dot>",
            KeyCode::KEY_END => "<end>",
            KeyCode::KEY_ESC => "<esc>",
            KeyCode::KEY_F10 => "<f10>",
            KeyCode::KEY_F11 => "<f11>",
            KeyCode::KEY_F12 => "<f12>",
            KeyCode::KEY_F13 => "<f13>",
            KeyCode::KEY_F14 => "<f14>",
            KeyCode::KEY_F15 => "<f15>",
            KeyCode::KEY_F16 => "<f16>",
            KeyCode::KEY_F17 => "<f17>",
            KeyCode::KEY_F18 => "<f18>",
            KeyCode::KEY_F19 => "<f19>",
            KeyCode::KEY_F20 => "<f20>",
            KeyCode::KEY_F21 => "<f21>",
            KeyCode::KEY_F22 => "<f22>",
            KeyCode::KEY_F23 => "<f23>",
            KeyCode::KEY_F24 => "<f24>",
            KeyCode::KEY_TAB => "<tab>",
            KeyCode::KEY_BACK => "<back>",
            KeyCode::KEY_DOWN => "<down>",
            KeyCode::KEY_HOME => "<home>",
            KeyCode::KEY_LEFT => "<left>",
            KeyCode::KEY_MODE => "<mode>",
            KeyCode::KEY_MOVE => "<move>",
            KeyCode::KEY_ANGLE => "<angle>",
            KeyCode::KEY_COMMA => "<comma>",
            KeyCode::KEY_ENTER => "<enter>",
            KeyCode::KEY_EQUAL => "<equal>",
            KeyCode::KEY_GRAVE => "<grave>",
            KeyCode::KEY_MINUS => "<minus>",
            KeyCode::KEY_RIGHT => "<right>",
            KeyCode::KEY_SLASH => "<slash>",
            KeyCode::KEY_DELETE => "<delete>",
            KeyCode::KEY_DOLLAR => "<dollar>",
            KeyCode::KEY_INSERT => "<insert>",
            KeyCode::KEY_PAGEUP => "<pageup>",
            KeyCode::KEY_LEFTALT => "<leftalt>",
            KeyCode::KEY_LEFT_UP => "<left_up>",
            KeyCode::KEY_CAPSLOCK => "<capslock>",
            KeyCode::KEY_LEFTCTRL => "<leftctrl>",
            KeyCode::KEY_LEFTMETA => "<leftmeta>",
            KeyCode::KEY_PAGEDOWN => "<pagedown>",
            KeyCode::KEY_RIGHTALT => "<rightalt>",
            KeyCode::KEY_RIGHT_UP => "<right_up>",
            KeyCode::KEY_BACKSLASH => "<backslash>",
            KeyCode::KEY_BACKSPACE => "<backspace>",
            KeyCode::KEY_LEFTBRACE => "<leftbrace>",
            KeyCode::KEY_LEFT_DOWN => "<left_down>",
            KeyCode::KEY_LEFTSHIFT => "<leftshift>",
            KeyCode::KEY_RIGHTCTRL => "<rightctrl>",
            KeyCode::KEY_RIGHTMETA => "<rightmeta>",
            KeyCode::KEY_SEMICOLON => "<semicolon>",
            KeyCode::KEY_APOSTROPHE => "<apostrophe>",
            KeyCode::KEY_RIGHTBRACE => "<rightbrace>",
            KeyCode::KEY_RIGHT_DOWN => "<right_down>",
            KeyCode::KEY_RIGHTSHIFT => "<rightshift>",
            KeyCode::UNKNOWN => "<???>",
        }
    }

    pub fn is_control(&self) -> bool {
        match self {
            KeyCode::KEY_F1
            | KeyCode::KEY_F2
            | KeyCode::KEY_F3
            | KeyCode::KEY_F4
            | KeyCode::KEY_F5
            | KeyCode::KEY_F6
            | KeyCode::KEY_F7
            | KeyCode::KEY_F8
            | KeyCode::KEY_F9
            | KeyCode::KEY_FN
            | KeyCode::KEY_UP
            | KeyCode::KEY_END
            | KeyCode::KEY_ESC
            | KeyCode::KEY_F10
            | KeyCode::KEY_F11
            | KeyCode::KEY_F12
            | KeyCode::KEY_F13
            | KeyCode::KEY_F14
            | KeyCode::KEY_F15
            | KeyCode::KEY_F16
            | KeyCode::KEY_F17
            | KeyCode::KEY_F18
            | KeyCode::KEY_F19
            | KeyCode::KEY_F20
            | KeyCode::KEY_F21
            | KeyCode::KEY_F22
            | KeyCode::KEY_F23
            | KeyCode::KEY_F24
            | KeyCode::KEY_BACK
            | KeyCode::KEY_DOWN
            | KeyCode::KEY_HOME
            | KeyCode::KEY_LEFT
            | KeyCode::KEY_MODE
            | KeyCode::KEY_MOVE
            | KeyCode::KEY_ENTER
            | KeyCode::KEY_RIGHT
            | KeyCode::KEY_DELETE
            | KeyCode::KEY_DOLLAR
            | KeyCode::KEY_INSERT
            | KeyCode::KEY_PAGEUP
            | KeyCode::KEY_LEFTALT
            | KeyCode::KEY_LEFT_UP
            | KeyCode::KEY_CAPSLOCK
            | KeyCode::KEY_LEFTCTRL
            | KeyCode::KEY_LEFTMETA
            | KeyCode::KEY_PAGEDOWN
            | KeyCode::KEY_RIGHTALT
            | KeyCode::KEY_RIGHT_UP
            | KeyCode::KEY_BACKSPACE
            | KeyCode::KEY_LEFT_DOWN
            | KeyCode::KEY_LEFTSHIFT
            | KeyCode::KEY_RIGHTCTRL
            | KeyCode::KEY_RIGHTMETA
            | KeyCode::KEY_RIGHT_DOWN
            | KeyCode::KEY_RIGHTSHIFT
            | KeyCode::UNKNOWN => true,
            _ => false,
        }
    }
}

impl FromStr for KeyCode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "0" => KeyCode::KEY_0,
            "1" => KeyCode::KEY_1,
            "2" => KeyCode::KEY_2,
            "3" => KeyCode::KEY_3,
            "4" => KeyCode::KEY_4,
            "5" => KeyCode::KEY_5,
            "6" => KeyCode::KEY_6,
            "7" => KeyCode::KEY_7,
            "8" => KeyCode::KEY_8,
            "9" => KeyCode::KEY_9,
            "a" => KeyCode::KEY_A,
            "b" => KeyCode::KEY_B,
            "c" => KeyCode::KEY_C,
            "d" => KeyCode::KEY_D,
            "e" => KeyCode::KEY_E,
            "f" => KeyCode::KEY_F,
            "g" => KeyCode::KEY_G,
            "h" => KeyCode::KEY_H,
            "i" => KeyCode::KEY_I,
            "j" => KeyCode::KEY_J,
            "k" => KeyCode::KEY_K,
            "l" => KeyCode::KEY_L,
            "m" => KeyCode::KEY_M,
            "n" => KeyCode::KEY_N,
            "o" => KeyCode::KEY_O,
            "p" => KeyCode::KEY_P,
            "q" => KeyCode::KEY_Q,
            "r" => KeyCode::KEY_R,
            "s" => KeyCode::KEY_S,
            "t" => KeyCode::KEY_T,
            "u" => KeyCode::KEY_U,
            "v" => KeyCode::KEY_V,
            "w" => KeyCode::KEY_W,
            "x" => KeyCode::KEY_X,
            "y" => KeyCode::KEY_Y,
            "z" => KeyCode::KEY_Z,
            " " => KeyCode::KEY_SPACE,
            "<f1>" => KeyCode::KEY_F1,
            "<f2>" => KeyCode::KEY_F2,
            "<f3>" => KeyCode::KEY_F3,
            "<f4>" => KeyCode::KEY_F4,
            "<f5>" => KeyCode::KEY_F5,
            "<f6>" => KeyCode::KEY_F6,
            "<f7>" => KeyCode::KEY_F7,
            "<f8>" => KeyCode::KEY_F8,
            "<f9>" => KeyCode::KEY_F9,
            "<fn>" => KeyCode::KEY_FN,
            "<up>" => KeyCode::KEY_UP,
            "<dot>" => KeyCode::KEY_DOT,
            "<end>" => KeyCode::KEY_END,
            "<esc>" => KeyCode::KEY_ESC,
            "<f10>" => KeyCode::KEY_F10,
            "<f11>" => KeyCode::KEY_F11,
            "<f12>" => KeyCode::KEY_F12,
            "<f13>" => KeyCode::KEY_F13,
            "<f14>" => KeyCode::KEY_F14,
            "<f15>" => KeyCode::KEY_F15,
            "<f16>" => KeyCode::KEY_F16,
            "<f17>" => KeyCode::KEY_F17,
            "<f18>" => KeyCode::KEY_F18,
            "<f19>" => KeyCode::KEY_F19,
            "<f20>" => KeyCode::KEY_F20,
            "<f21>" => KeyCode::KEY_F21,
            "<f22>" => KeyCode::KEY_F22,
            "<f23>" => KeyCode::KEY_F23,
            "<f24>" => KeyCode::KEY_F24,
            "<tab>" => KeyCode::KEY_TAB,
            "<back>" => KeyCode::KEY_BACK,
            "<down>" => KeyCode::KEY_DOWN,
            "<home>" => KeyCode::KEY_HOME,
            "<left>" => KeyCode::KEY_LEFT,
            "<mode>" => KeyCode::KEY_MODE,
            "<move>" => KeyCode::KEY_MOVE,
            "<angle>" => KeyCode::KEY_ANGLE,
            "<comma>" => KeyCode::KEY_COMMA,
            "<enter>" => KeyCode::KEY_ENTER,
            "<equal>" => KeyCode::KEY_EQUAL,
            "<grave>" => KeyCode::KEY_GRAVE,
            "<minus>" => KeyCode::KEY_MINUS,
            "<right>" => KeyCode::KEY_RIGHT,
            "<slash>" => KeyCode::KEY_SLASH,
            "<delete>" => KeyCode::KEY_DELETE,
            "<dollar>" => KeyCode::KEY_DOLLAR,
            "<insert>" => KeyCode::KEY_INSERT,
            "<pageup>" => KeyCode::KEY_PAGEUP,
            "<leftalt>" => KeyCode::KEY_LEFTALT,
            "<left_up>" => KeyCode::KEY_LEFT_UP,
            "<capslock>" => KeyCode::KEY_CAPSLOCK,
            "<leftctrl>" => KeyCode::KEY_LEFTCTRL,
            "<leftmeta>" => KeyCode::KEY_LEFTMETA,
            "<pagedown>" => KeyCode::KEY_PAGEDOWN,
            "<rightalt>" => KeyCode::KEY_RIGHTALT,
            "<right_up>" => KeyCode::KEY_RIGHT_UP,
            "<backslash>" => KeyCode::KEY_BACKSLASH,
            "<backspace>" => KeyCode::KEY_BACKSPACE,
            "<leftbrace>" => KeyCode::KEY_LEFTBRACE,
            "<left_down>" => KeyCode::KEY_LEFT_DOWN,
            "<leftshift>" => KeyCode::KEY_LEFTSHIFT,
            "<rightctrl>" => KeyCode::KEY_RIGHTCTRL,
            "<rightmeta>" => KeyCode::KEY_RIGHTMETA,
            "<semicolon>" => KeyCode::KEY_SEMICOLON,
            "<apostrophe>" => KeyCode::KEY_APOSTROPHE,
            "<rightbrace>" => KeyCode::KEY_RIGHTBRACE,
            "<right_down>" => KeyCode::KEY_RIGHT_DOWN,
            "<rightshift>" => KeyCode::KEY_RIGHTSHIFT,
            "<???>" => KeyCode::UNKNOWN,
            code => bail!("failed to parse keycode: {}", code),
        })
    }
}
