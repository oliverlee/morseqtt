use crate::word::{ParseWordError, Word};
use std::fmt;
use std::str::FromStr;

pub struct Phrase {
    words: Vec<Word>,
}

impl FromStr for Phrase {
    type Err = ParseWordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Self { words })
    }
}

impl fmt::Display for Phrase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.words
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join("   ") // Use a wider gap between words
        )
    }
}

#[cfg(test)]
mod test {
    use super::Phrase;

    #[test]
    fn parse() {
        assert_eq!(
            "MORSE CODE".parse::<Phrase>().unwrap().to_string(),
            "-- --- .-. ... .   -.-. --- -.. ."
        );
    }
}
