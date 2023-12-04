use std::{collections::HashMap, io::BufRead};

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let lines = input.read().lines();

    let mut sum = 0;

    for line in lines {
        let line = line.unwrap();

        let first_digit = find_first_digit(&line);

        let line_reversed: String = line.chars().rev().collect();
        let last_digit = find_last_digit(&line_reversed);

        let number = first_digit * 10 + last_digit;

        sum += number;
    }

    sum
}

/// Find the first digit in the `line`.
fn find_first_digit(line: &str) -> i64 {
    let str_to_number: HashMap<&str, i64> = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into();

    for i in 0..line.len() {
        let slice = &line[i..];

        for key in str_to_number.keys() {
            if slice.starts_with(key) {
                return *str_to_number.get(key).unwrap();
            }
        }
    }

    unreachable!()
}

/// Find the last digit in the `line`.
fn find_last_digit(line: &str) -> i64 {
    let str_to_number: HashMap<&str, i64> = [
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("eno", 1),
        ("owt", 2),
        ("eerht", 3),
        ("ruof", 4),
        ("evif", 5),
        ("xis", 6),
        ("neves", 7),
        ("thgie", 8),
        ("enin", 9),
    ]
    .into();

    for i in 0..line.len() {
        let slice = &line[i..];

        for key in str_to_number.keys() {
            if slice.starts_with(key) {
                return *str_to_number.get(key).unwrap();
            }
        }
    }

    unreachable!()
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
        let digits = String::from("one5azd9eight");

        assert_eq!(solve(&Input::from(digits)), 18);
    }

    #[test]
    fn simple() {
        let digits = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        assert_eq!(solve(&Input::from(digits)), 281);
    }

    #[test]
    fn eat_word() {
        let digits = String::from("eightwo");

        assert_eq!(solve(&Input::from(digits)), 82);
    }
}
