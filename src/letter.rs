use crate::mark::Mark;
use crate::timing::Signal;
use std::collections::HashSet;
use std::convert::{From, TryFrom};
use std::fmt;

lazy_static! {
    static ref VALID_LETTERS: HashSet<char> = {
        let mut s = HashSet::new();
        s.insert('A');
        s.insert('B');
        s.insert('C');
        s.insert('D');
        s.insert('E');
        s.insert('F');
        s.insert('G');
        s.insert('H');
        s.insert('I');
        s.insert('J');
        s.insert('K');
        s.insert('L');
        s.insert('M');
        s.insert('N');
        s.insert('O');
        s.insert('P');
        s.insert('Q');
        s.insert('R');
        s.insert('S');
        s.insert('T');
        s.insert('U');
        s.insert('V');
        s.insert('W');
        s.insert('X');
        s.insert('Y');
        s.insert('Z');
        s.insert('0');
        s.insert('1');
        s.insert('2');
        s.insert('3');
        s.insert('4');
        s.insert('5');
        s.insert('6');
        s.insert('7');
        s.insert('8');
        s.insert('9');
        s.insert('&');
        s.insert('\'');
        s.insert('@');
        s.insert(')');
        s.insert('(');
        s.insert(':');
        s.insert(',');
        s.insert('=');
        s.insert('!');
        s.insert('.');
        s.insert('-');
        s.insert('+');
        s.insert('"');
        s.insert('?');
        s.insert('/');
        s
    };
}

#[derive(Debug, PartialEq)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Ampersand,
    Apostrophe,
    At,
    BracketClose,
    BracketOpen,
    Colon,
    Comma,
    Equal,
    Bang,
    Period,
    Hyphen,
    Plus,
    Quote,
    Query,
    Slash,
}

impl Letter {
    pub fn str_ref(&self) -> &'static str {
        match self {
            Self::A => ".-",
            Self::B => "-...",
            Self::C => "-.-.",
            Self::D => "-..",
            Self::E => ".",
            Self::F => "..-.",
            Self::G => "--.",
            Self::H => "....",
            Self::I => "..",
            Self::J => ".---",
            Self::K => "-.-",
            Self::L => ".-..",
            Self::M => "--",
            Self::N => "-.",
            Self::O => "---",
            Self::P => ".--.",
            Self::Q => "--.-",
            Self::R => ".-.",
            Self::S => "...",
            Self::T => "-",
            Self::U => "..-",
            Self::V => "...-",
            Self::W => ".--",
            Self::X => "-..-",
            Self::Y => "-.--",
            Self::Z => "--..",
            Self::Digit0 => "-----",
            Self::Digit1 => ".----",
            Self::Digit2 => "..---",
            Self::Digit3 => "...--",
            Self::Digit4 => "....-",
            Self::Digit5 => ".....",
            Self::Digit6 => "-....",
            Self::Digit7 => "--...",
            Self::Digit8 => "---..",
            Self::Digit9 => "----.",
            Self::Ampersand => ".-...",
            Self::Apostrophe => ".----.",
            Self::At => ".--.-.",
            Self::BracketClose => "-.--.-",
            Self::BracketOpen => "-.--.",
            Self::Colon => "---...",
            Self::Comma => "--..--",
            Self::Equal => "-...-",
            Self::Bang => "-.-.--",
            Self::Period => ".-.-.-",
            Self::Hyphen => "-....-",
            Self::Plus => ".-.-.",
            Self::Quote => ".-..-.",
            Self::Query => "..--..",
            Self::Slash => "-..-.",
        }
    }

    pub fn marks(&self) -> Vec<Mark> {
        self.str_ref().chars().map(Mark::from).collect()
    }

    pub fn timing(&self) -> impl Iterator<Item = Signal> {
        self.marks()
            .into_iter()
            .flat_map(|m| std::iter::once(Signal::Off).chain(m.timing()))
            .skip(1) // Ignore the first mark gap
    }
}

impl From<char> for Letter {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'E' => Self::E,
            'F' => Self::F,
            'G' => Self::G,
            'H' => Self::H,
            'I' => Self::I,
            'J' => Self::J,
            'K' => Self::K,
            'L' => Self::L,
            'M' => Self::M,
            'N' => Self::N,
            'O' => Self::O,
            'P' => Self::P,
            'Q' => Self::Q,
            'R' => Self::R,
            'S' => Self::S,
            'T' => Self::T,
            'U' => Self::U,
            'V' => Self::V,
            'W' => Self::W,
            'X' => Self::X,
            'Y' => Self::Y,
            'Z' => Self::Z,
            '0' => Self::Digit0,
            '1' => Self::Digit1,
            '2' => Self::Digit2,
            '3' => Self::Digit3,
            '4' => Self::Digit4,
            '5' => Self::Digit5,
            '6' => Self::Digit6,
            '7' => Self::Digit7,
            '8' => Self::Digit8,
            '9' => Self::Digit9,
            '&' => Self::Ampersand,
            '\'' => Self::Apostrophe,
            '@' => Self::At,
            ')' => Self::BracketClose,
            '(' => Self::BracketOpen,
            ':' => Self::Colon,
            ',' => Self::Comma,
            '=' => Self::Equal,
            '!' => Self::Bang,
            '.' => Self::Period,
            '-' => Self::Hyphen,
            '+' => Self::Plus,
            '"' => Self::Quote,
            '?' => Self::Query,
            '/' => Self::Slash,
            _ => panic!("Unexpected char for Letter"),
        }
    }
}

impl TryFrom<&char> for Letter {
    type Error = &'static str;

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        if VALID_LETTERS.contains(c) {
            Ok(Self::from(*c))
        } else {
            Err("Invalid char for Letter")
        }
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.str_ref())
    }
}

#[cfg(test)]
mod test {
    use super::{Letter, VALID_LETTERS};
    use crate::mark::Mark;
    use std::convert::{From, TryFrom};

    #[test]
    #[should_panic]
    fn invalid_char_a() {
        Letter::from('a');
    }

    #[test]
    #[should_panic]
    fn invalid_char_space() {
        Letter::from(' ');
    }

    #[test]
    #[allow(non_snake_case)]
    fn display_A() {
        assert_eq!(Letter::from('A').to_string(), ".-");
    }

    #[test]
    #[allow(non_snake_case)]
    fn from_M() {
        assert_eq!(Letter::from('M').to_string(), "--");
    }

    #[test]
    fn from_0() {
        assert_eq!(Letter::from('0').to_string(), "-----");
    }

    #[test]
    fn from_period() {
        assert_eq!(Letter::from('.').to_string(), ".-.-.-");
    }

    #[test]
    fn from_apostrophe() {
        assert_eq!(Letter::from('\'').to_string(), ".----.");
    }

    #[test]
    fn as_marks() {
        assert_eq!(Letter::M.marks(), vec![Mark::Dash, Mark::Dash]);
    }

    #[test]
    #[allow(non_snake_case)]
    fn try_from_M() {
        assert_eq!(Letter::try_from('M').unwrap().to_string(), "--");
    }

    #[test]
    fn valid_char() {
        assert!(VALID_LETTERS.contains(&'M'));
        assert!(!VALID_LETTERS.contains(&'m'));
        assert!(!VALID_LETTERS.contains(&' '));
    }

    #[test]
    fn try_from_m() {
        assert!(Letter::try_from(&'m').is_err());
    }

    #[test]
    fn try_from_space() {
        assert!(Letter::try_from(&' ').is_err());
    }
}
