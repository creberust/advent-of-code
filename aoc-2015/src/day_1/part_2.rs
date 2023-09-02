use std::io::BufRead;

use common::{Input, Solution};

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) -> i64 {
        let mut result = 0;
        let mut position = 1;

        for line in input.read().lines() {
            let line = line.unwrap();

            for c in line.chars() {
                match c {
                    '(' => result += 1,
                    ')' => result -= 1,
                    _ => continue,
                }

                if result < 0 {
                    break;
                }

                position += 1;
            }
        }

        position
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
    fn one() {
        solve(")", 1);
    }

    #[test]
    fn five() {
        solve("()())", 5);
    }
}