use crate::timing::Signal;
use std::convert;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Mark {
    Dot,
    Dash,
}

impl Mark {
    pub fn timing(&self) -> impl Iterator<Item = Signal> {
        match self {
            Self::Dot => std::iter::repeat(Signal::On).take(1),
            Self::Dash => std::iter::repeat(Signal::On).take(3),
        }
    }
}

impl convert::From<char> for Mark {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Dot,
            '-' => Self::Dash,
            _ => panic!("Unexpected char for Mark"),
        }
    }
}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Dot => ".",
                Self::Dash => "-",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::Mark;

    #[test]
    fn display_dot() {
        assert_eq!(Mark::Dot.to_string(), ".");
    }

    #[test]
    fn display_dash() {
        assert_eq!(Mark::Dash.to_string(), "-");
    }

    #[test]
    #[should_panic]
    fn invalid_char() {
        Mark::from('a');
    }

    #[test]
    fn convert_dot() {
        assert_eq!(Mark::from('.').to_string(), ".");
    }

    #[test]
    fn convert_dash() {
        assert_eq!(Mark::from('-').to_string(), "-");
    }
}
