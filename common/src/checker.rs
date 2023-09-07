//! Implementation for a Checker for a Solution.

use crate::{Input, Solution};

#[macro_export]
macro_rules! check {
    ($input:expr, $expected:expr) => {
        CHECKER.check($input, $expected);
    };
}

/// Checker helper to test solution easily.
///
/// # Example
///
/// ```
/// use crate::{Checker, check};
/// # use crate::Input;
///
/// fn solve(input: &Input) -> i64 {
///     todo!()
/// }
///
/// #[cfg(test)]
/// mod tests {
///     // Create the CHECKER (this name is mandatory).
///     const CHECKER: Checker = Checker::new(solve);
///
///     #[test]
///     fn simple() {
///         // Set your input and your expected result.
///         check!("my_input", 42);
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Checker {
    sol: Solution,
}

impl Checker {
    /// Create a new Checker for a Solution.
    pub const fn new(sol: Solution) -> Self {
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
