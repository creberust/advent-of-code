use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let lines = input.read().lines();

    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();

        let first_digit = line.chars().find(|c| char::is_numeric(*c)).unwrap();
        let last_digit = line.chars().rev().find(|c| char::is_numeric(*c)).unwrap();

        let number = first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();

        sum += number;
    }

    sum as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        let zero = String::from("zad0azdazd");

        assert_eq!(solve(&Input::from(zero)), 0);
    }

    #[test]
    fn two_digit() {
        let digits = String::from("zad5azd9azd");

        assert_eq!(solve(&Input::from(digits)), 59);
    }
}
