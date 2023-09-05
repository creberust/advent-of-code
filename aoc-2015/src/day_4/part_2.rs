use std::io::BufRead;

use common::{Input, Solution};

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve(input: impl AsRef<str>, expected: i64) {
        // Given
        let input = Input::Text(String::from(input.as_ref()));

        // When
        let result = Solver.solve(&input);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn simple() {}
}
