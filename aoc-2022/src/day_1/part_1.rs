use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

pub fn part_1(input: &Path) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let calories = parse_max_calories(reader);

    println!("Max calories: {}", calories);
}

fn parse_max_calories<R: Read>(mut input: BufReader<R>) -> u32 {
    let mut calories = 1;
    let mut max_calories: u32 = 0;

    while calories != 0 {
        calories = parse_calories(&mut input);

        max_calories = max_calories.max(calories);
    }

    max_calories
}

fn parse_calories<R: Read>(input: &mut BufReader<R>) -> u32 {
    let mut calories: u32 = 0;

    for line in input.lines() {
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
    use std::io::Cursor;

    use super::*;

    #[test]
    fn zero_calory() {
        let zero = String::from("0");
        let cursor = Cursor::new(zero);

        assert_eq!(parse_calories(&mut BufReader::new(cursor)), 0);
    }

    #[test]
    fn one_thousand_calory() {
        let calory = String::from("1000");
        let cursor = Cursor::new(calory);

        assert_eq!(parse_calories(&mut BufReader::new(cursor)), 1000);
    }

    #[test]
    fn one_hundred_calories() {
        let calories = String::from("20\n50\n30");
        let cursor = Cursor::new(calories);

        assert_eq!(parse_calories(&mut BufReader::new(cursor)), 100);
    }

    #[test]
    fn two_elves_carrying_same_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60");
        let cursor = Cursor::new(calories);

        assert_eq!(parse_max_calories(BufReader::new(cursor)), 100);
    }

    #[test]
    fn two_elves_second_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60\n10");
        let cursor = Cursor::new(calories);

        assert_eq!(parse_max_calories(BufReader::new(cursor)), 110);
    }

    #[test]
    fn three_elves_first_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n40\n\n10");
        let cursor = Cursor::new(calories);

        assert_eq!(parse_max_calories(BufReader::new(cursor)), 100);
    }
}
