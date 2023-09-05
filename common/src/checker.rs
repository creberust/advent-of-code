//! Implementation for a Checker for a Solution.

use crate::{Input, Solution};

/// Checker helper to test solution easily.
#[derive(Debug)]
pub struct Checker {
    sol: Solution,
}

impl Checker {
    /// Create a new Checker for a Solution.
    pub fn new(sol: Solution) -> Self {
        Self { sol }
    }

    /// Check for a solution with a given `input` with a result `expected`.
    pub fn check(&self, input: impl AsRef<str>, expected: i64) {
        // Given
        let input = Input::Text(String::from(input.as_ref()));

        // When
        let result = (self.sol)(&input);

        // Then
        assert_eq!(result, expected);
    }
}
