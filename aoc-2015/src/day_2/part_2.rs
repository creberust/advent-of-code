use std::io::BufRead;

use common::{Input, Solution};

use super::both::Present;

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) -> i64 {
        let mut feet_of_ribon = 0;

        for line in input.read().lines() {
            let line = line.unwrap();

            let present = Present::from(line.as_str());

            feet_of_ribon += present.volume() + present.smallest_perimeter();
        }

        feet_of_ribon as i64
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
