use crate::code::word::{ParseWordError, Word};
use crate::timing::Signal;
use std::fmt;
use std::str::FromStr;

pub struct Phrase {
    words: Vec<Word>,
}

impl Phrase {
    pub fn timing<'a>(&'a self) -> impl Iterator<Item = Signal> + 'a {
        self.words
            .iter()
            .flat_map(|w| std::iter::repeat(Signal::Off).take(7).chain(w.timing()))
            .skip(7) // Ignore the first word gap
    }
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

    #[test]
    fn timing() {
        assert_eq!(
            "MORSE CODE"
                .parse::<Phrase>()
                .unwrap()
                .timing()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(""),
            "===.===...===.===.===...=.===.=...=.=.=...=.......===.=.===.=...===.===.===...===.=.=...="
        );
    }
}
