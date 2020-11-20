use anyhow::*;
use serde::de::Error;

use crate::keyboard::key_code::KeyCode;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct OutputChar {
    pub key: KeyCode,
    pub is_upper: bool,
}

impl<'de> serde::Deserialize<'de> for OutputChar {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let c = char::deserialize(deserializer)?;
        OutputChar::from_char(c).map_err(|e| D::Error::custom(e))
    }
}

impl From<KeyCode> for OutputChar {
    fn from(key: KeyCode) -> Self {
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
        let key = c
            .to_ascii_lowercase()
            .to_string()
            .parse()
            .context("Error parsing output character")?;
        Ok(OutputChar {
            key,
            is_upper: !key.is_control() && c.is_uppercase(),
        })
    }

    //pub fn from_str(s: &str) -> Result<OutputChar> {
    //let key = s
    //.parse()
    //.with_context(|| format!("Error parsing output character in string {}", s))?;
    //Ok(OutputChar {
    //key,
    //is_upper: !key.is_control() && s.chars().next().unwrap().is_uppercase(),
    //})
    //}
}

pub fn output_chars_from_string(s: &str) -> Result<Vec<OutputChar>> {
    s.chars().map(|x| OutputChar::from_char(x)).collect()
}
