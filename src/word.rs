use crate::letter::Letter;
use std::convert::TryFrom;
use std::error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseWordError {
    c: char,
}

impl fmt::Display for ParseWordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Morse Code letter: {}", self.c)
    }
}

impl error::Error for ParseWordError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct Word {
    letters: Vec<Letter>,
}

impl FromStr for Word {
    type Err = ParseWordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let letters = s
            .to_uppercase()
            .chars()
            .map(|c| Letter::try_from(&c).or(Err(ParseWordError { c })))
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Self { letters })
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.letters
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[cfg(test)]
mod test {
    use super::Word;
    use crate::letter::Letter;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let w = Word::from_str("MORSE").unwrap();
        assert_eq!(
            w.letters,
            vec![Letter::M, Letter::O, Letter::R, Letter::S, Letter::E]
        );
    }

    #[test]
    fn from_str_lower() {
        let w = Word::from_str("morse").unwrap();
        assert_eq!(
            w.letters,
            vec![Letter::M, Letter::O, Letter::R, Letter::S, Letter::E]
        );
    }

    #[test]
    fn parse() {
        let w: Word = "MORSE".parse().unwrap();
        assert_eq!(
            w.letters,
            vec![Letter::M, Letter::O, Letter::R, Letter::S, Letter::E]
        );
    }

    #[test]
    fn display() {
        let w: Word = "MORSE".parse().unwrap();
        assert_eq!(w.to_string(), "-- --- .-. ... .");
    }

    #[test]
    fn invalid_char() {
        assert!(Word::from_str("MORSE ").is_err());
    }
}
