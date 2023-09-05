//! Implementation for a Solver for a Solution.

use crate::{solution::Solution, Input};

pub struct Solver {
    f: Solution,
}

impl Solver {
    /// Create a new Solver with the given `f`.
    pub fn new(f: Solution) -> Self {
        Self { f }
    }

    /// Call the solver with `input` and return the result.
    pub fn solve(&self, input: &Input) -> i64 {
        (self.f)(input)
    }
}
