use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;

pub fn part_2(input: &Path) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let calories = parse_max_calories(reader);

    assert_eq!(calories.len(), 3);
    println!("Max calories: {}", calories.iter().sum::<u32>());
}

fn parse_max_calories<R: Read>(mut input: BufReader<R>) -> Vec<u32> {
    let mut heap = BinaryHeap::<u32>::new();

    loop {
        let calories = parse_calories(&mut input);
        heap.push(calories);

        if calories == 0 {
            break; // EOF
        }
    }

    assert!(heap.len() > 2);

    let mut max_calories = Vec::new();

    for _ in 0..3 {
        max_calories.push(heap.pop().unwrap());
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
        let line = Cursor::new(zero.as_bytes());

        assert_eq!(parse_calories(&mut BufReader::new(line)), 0);
    }

    #[test]
    fn one_thousand_calory() {
        let calory = String::from("1000");
        let line = Cursor::new(calory.as_bytes());

        assert_eq!(parse_calories(&mut BufReader::new(line)), 1000);
    }

    #[test]
    fn one_hundred_calories() {
        let calories = String::from("20\n50\n30");
        let line = Cursor::new(calories.as_bytes());

        assert_eq!(parse_calories(&mut BufReader::new(line)), 100);
    }

    #[test]
    fn four_elves_carrying_same_calories() {
        let calories = String::from("20\n50\n30\n\n40\n60\n\n10\n\n0");
        let line = Cursor::new(calories.as_bytes());

        assert_eq!(
            parse_max_calories(BufReader::new(line)).iter().sum::<u32>(),
            210
        );
    }

    #[test]
    fn four_elves_second_carrying_more_calories() {
        let calories = String::from("20\n50\n30\n\n100\n40\n60\n10\n\n250\n\n20");
        let line = Cursor::new(calories.as_bytes());

        assert_eq!(
            parse_max_calories(BufReader::new(line)).iter().sum::<u32>(),
            560
        );
    }
}
