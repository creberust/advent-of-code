use std::io::{BufRead, Lines};

use common::Input;

pub fn part_1(input: &Input) {
    let calories = parse_max_calories(input);

    println!("{}", calories);
}

fn parse_max_calories(input: &Input) -> u32 {
    let mut calories = 1;
    let mut max_calories: u32 = 0;

    let mut lines = input.read().lines();

    while calories != 0 {
        calories = parse_calories(&mut lines);

        max_calories = max_calories.max(calories);
    }

    max_calories
}

fn parse_calories<B: BufRead>(lines: &mut Lines<B>) -> u32 {
    let mut calories: u32 = 0;

    for line in lines {
        let line = line.unwrap();

        if line.is_empty() {
            break; // Empty line between two elves
        }

        calories += line.parse::<u32>().unwrap();
    }

    calories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_calory() {
        let zero = String::from("0");

        assert_eq!(parse_calories(&mut Input::from(zero).read().lines()), 0);
    }

    #[test]
    fn one_thousand_calory() {
        let calory = String::from("1000");

        assert_eq!(
            parse_calories(&mut Input::from(calory).read().lines()),
            1000
        );
    }

    #[test]
    fn one_hundred_calories() {
        let calories = String::from("20\n50\n30");

        assert_eq!(
            parse_calories(&mut Input::from(calories).read().lines()),
            100
        );
    }

    #[test]
    fn two_elves_carrying_same_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60");

        assert_eq!(parse_max_calories(&Input::from(calories)), 100);
    }

    #[test]
    fn two_elves_second_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60\n10");

        assert_eq!(parse_max_calories(&Input::from(calories)), 110);
    }

    #[test]
    fn three_elves_first_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n40\n\n10");

        assert_eq!(parse_max_calories(&Input::from(calories)), 100);
    }
}
