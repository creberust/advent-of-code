//! Implementation of Year for Advent of Code.

use std::fmt::Display;

/// The year for the Advent of Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Year(u16);

impl Year {
    /// The minimum year of the Advent of Code event.
    const MIN: u16 = 2015;
    /// The maximum recent year of the Advent of Code event.
    const MAX: u16 = 2023;

    /// The first year of the Advent of Code event.
    pub const FIRST: Year = Year(Self::MIN);
    /// The last year of the Advent of Code event.
    pub const LAST: Year = Year(Self::MAX);
}

impl Year {
    /// Get the year number.
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Year {
    fn from(value: u16) -> Self {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            panic!(
                "Invalid value for Year: {} ∉ [{}, {}]",
                value,
                Self::MIN,
                Self::MAX,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_year() {
        for year in Year::MIN..=Year::MAX {
            assert_eq!(Year::from(year), Year(year));
        }
    }

    #[test]
    #[should_panic]
    fn invalid_year_zero() {
        let _ = Year::from(Year::MIN - 1);
    }

    #[test]
    #[should_panic]
    fn invalid_year_twenty_six() {
        let _ = Year::from(Year::MAX + 1);
    }
}
