//! Abstraction for a Solution.

use crate::*;

/// Functions that a solution needs to implement.
pub trait Solution {
    /// Solve the solution with the given `input` and return the result.
    fn solve(&self, input: &Input) -> u32;
}
