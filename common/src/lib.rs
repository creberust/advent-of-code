/// The Advent of Code year
pub struct Year(u16);

/// The day of the puzzle
pub struct Day(u8);

/// The part of the puzzle to solve
pub enum Part {
    /// Only the first part
    One,
    /// Only the second part
    Two,
    /// Both of the parts
    Both,
}

/// Abstraction for an AoC event (year)
pub trait Event {
    /// Solve the given day for an AoC event
    ///
    /// # Parameters
    /// * `day` - The day of the event to solve
    /// * `part` - The part(s) of the puzzle to solve
    fn solve(day: Day, part: Part);
}

/// Abstraction for an AoC puzzle
pub trait Puzzle {
    /// Solve the `part` of the puzzle
    ///
    /// # Parameters
    /// * `part` - The part(s) of the puzzle to solve
    fn solve(part: Part);
}
