use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: impl AsRef<str>, expected: i64) {
        // Given
        let input = Input::Text(String::from(input.as_ref()));

        // When
        let result = solve(&input);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn zero() {
        check("", 0);
        check("(())", 0);
        check("()()", 0);
    }

    #[test]
    fn three() {
        check("(((", 3);
        check("(()(()(", 3);
        check("))(((((", 3);
    }

    #[test]
    fn minus_one() {
        check("())", -1);
        check("))(", -1);
    }

    #[test]
    fn minus_three() {
        check(")))", -3);
        check(")())())", -3);
    }
}
