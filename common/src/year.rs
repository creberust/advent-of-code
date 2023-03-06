use std::fmt::Display;

/// The Advent of Code year
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Year(pub u16);

impl Year {
    /// The first year of the Advent of Code event
    pub const MIN: u16 = 2015;
    /// The most recent year of the Advent of Code event
    pub const MAX: u16 = 2022;
}

impl From<u16> for Year {
    fn from(value: u16) -> Self {
        if value < Self::MIN || value > Self::MAX {
            panic!(
                "The year isn't within the range [{}, {}]: {}",
                Self::MIN,
                Self::MAX,
                value
            );
        }

        Year(value)
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
