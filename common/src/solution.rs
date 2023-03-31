use crate::*;

/// Functions that a solution needs to implement
pub trait Solution {
    /// Solve the solution with the given input
    ///
    /// # Parameters
    /// * `input` - The input for the solver
    fn solve(&self, input: &Input);
}
