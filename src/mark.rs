use std::convert;
use std::fmt;

pub enum Mark {
    Dot,
    Dash,
}

impl convert::From<char> for Mark {
    fn from(c: char) -> Self {
        match c {
            '.' => Mark::Dot,
            '-' => Mark::Dash,
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
                Mark::Dot => ".",
                Mark::Dash => "-",
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
