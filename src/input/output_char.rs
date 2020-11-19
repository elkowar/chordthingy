use anyhow::*;
use evdev_rs::enums::EV_KEY;
use serde::de::Error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OutputChar {
    pub key: EV_KEY,
    pub is_upper: bool,
}

impl<'de> serde::Deserialize<'de> for OutputChar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        OutputChar::from_str(&s).map_err(|e| D::Error::custom(e))
    }
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
    pub fn is_upper(&self) -> bool {
        self.is_upper
    }

    pub fn from_char(c: char) -> Result<OutputChar> {
        OutputChar::from_str(&c.to_string())
    }

    pub fn from_str(s: &str) -> Result<OutputChar> {
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
            "<bs>" => EV_KEY::KEY_BACKSPACE,
            c => bail!("Unrecognized character: {}", c),
        };

        Ok(OutputChar {
            key,
            is_upper: s.len() == 1 && s.chars().next().unwrap().is_uppercase(),
        })
    }
}

pub fn output_chars_from_string(s: &str) -> Result<Vec<OutputChar>> {
    s.chars().map(|x| OutputChar::from_char(x)).collect()
}
