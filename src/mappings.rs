use anyhow::*;
use std::collections::HashMap;

use crate::keyboard::{
    chord::Chord,
    output_char::{output_chars_from_string, OutputChar},
};

#[derive(Debug)]
pub struct Mappings {
    mappings: HashMap<Chord, Vec<OutputChar>>,
}

impl Mappings {
    pub fn from_reader<R: std::io::Read>(reader: R) -> Result<Self> {
        let mappings: HashMap<String, String> =
            serde_json::from_reader(reader).context("Failed to parse mappings")?;
        let mappings: HashMap<Chord, Vec<OutputChar>> = mappings
            .into_iter()
            .map(|(k, v)| Ok((Chord::from_string(k), output_chars_from_string(&v)?)))
            .collect::<Result<_>>()?;

        Ok(Mappings { mappings })
    }

    pub fn lookup(&self, chord: &Chord) -> Option<&Vec<OutputChar>> {
        self.mappings.get(chord)
    }
}
