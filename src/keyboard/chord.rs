use itertools::Itertools;

use crate::keyboard::key_code::KeyCode;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Chord(Vec<KeyCode>);

impl Chord {
    pub fn from_string<S: AsRef<str>>(s: S) -> Self {
        let pattern = regex::Regex::new("<.*?>|.").unwrap();
        Chord(
            pattern
                .find_iter(s.as_ref())
                .map(|part| part.as_str().to_owned().parse().unwrap_or(KeyCode::UNKNOWN))
                .sorted()
                .dedup()
                .collect(),
        )
    }

    pub fn len(&self) -> usize {
        self.0.iter().filter(|x| !x.is_control()).count()
    }
}

impl Chord {
    pub fn from_key_codes(keys: Vec<KeyCode>) -> Self {
        let parts: Vec<KeyCode> = keys.into_iter().sorted().dedup().collect_vec();
        Chord(parts)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    pub fn test_new_chord() {
        assert_eq!(
            Chord::from_string("<backspace>"),
            Chord(vec![KeyCode::KEY_BACKSPACE])
        );
        assert_eq!(Chord::from_string("a"), Chord(vec![KeyCode::KEY_A]));
        assert_eq!(
            Chord::from_string("aa<backspace>a"),
            Chord(vec![KeyCode::KEY_A, KeyCode::KEY_BACKSPACE])
        );

        assert_eq!(
            Chord::from_string("a b"),
            Chord(vec![KeyCode::KEY_A, KeyCode::KEY_B, KeyCode::KEY_SPACE])
        );
    }
}
