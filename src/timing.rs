use std::fmt;

#[derive(Copy, Clone)]
pub enum Signal {
    On,
    Off,
}

impl fmt::Display for Signal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::On => "=",
                Self::Off => ".",
            }
        )
    }
}
