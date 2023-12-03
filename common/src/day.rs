//! Implementation for a Day in Advent of Code.

use std::fmt::Display;

/// The day of the puzzle.
///
/// The day can only be in the range [[Day::MIN], [Day::MAX]].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Day(u8);

impl Day {
    /// The minimum day correspond to the 1st of December.
    const MIN: u8 = 1;
    /// The maximum day correspond to the 25th of December.
    const MAX: u8 = 25;

    /// The first day of the Advent of Code.
    pub const FIRST: Day = Day(Self::MIN);
    /// The last day of the Advent of Code.
    pub const LAST: Day = Day(Self::MAX);
}

impl Day {
    /// Get the day number.
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        if !(Self::MIN..=Self::MAX).contains(&value) {
            panic!(
                "Invalid value for Day: {} ∉ [{}, {}]",
                value,
                Self::MIN,
                Self::MAX,
            );
        }

        Day(value)
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_day() {
        for day in Day::MIN..=Day::MAX {
            assert_eq!(Day::from(day), Day(day));
        }
    }

    #[test]
    #[should_panic]
    fn invalid_day_zero() {
        let _ = Day::from(Day::MIN - 1);
    }

    #[test]
    #[should_panic]
    fn invalid_day_twenty_six() {
        let _ = Day::from(Day::MAX + 1);
    }
}
