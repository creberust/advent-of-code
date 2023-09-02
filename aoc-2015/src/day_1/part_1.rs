use std::io::BufRead;

use common::{Input, Solution};

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) -> i64 {
        let mut result = 0;

        for line in input.read().lines() {
            let line = line.unwrap();

            for c in line.chars() {
                match c {
                    '(' => result += 1,
                    ')' => result -= 1,
                    _ => continue,
                }
            }
        }

        result
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
    fn zero() {
        solve("", 0);
        solve("(())", 0);
        solve("()()", 0);
    }

    #[test]
    fn three() {
        solve("(((", 3);
        solve("(()(()(", 3);
        solve("))(((((", 3);
    }

    #[test]
    fn minus_one() {
        solve("())", -1);
        solve("))(", -1);
    }

    #[test]
    fn minus_three() {
        solve(")))", -3);
        solve(")())())", -3);
    }
}
