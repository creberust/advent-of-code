use std::{collections::HashSet, io::BufRead};

use common::Input;

pub fn part_2(input: &Input) {
    let mut lines = input.read().lines();
    let mut priority: u32 = 0;

    loop {
        // Break condition
        let first = lines.next();
        if first.is_none() {
            break;
        }

        let first = first.unwrap().unwrap();
        let second = lines.next().unwrap().unwrap();
        let third = lines.next().unwrap().unwrap();

        priority += compute(first, second, third);
    }

    println!("{}", priority);
}

fn compute(first: String, second: String, third: String) -> u32 {
    let inputs = [first, second, third];

    let mut rucksacks = [HashSet::new(), HashSet::new(), HashSet::new()];

    for i in 0..3 {
        inputs[i].chars().into_iter().for_each(|c| {
            let _ = rucksacks[i].insert(c);
            ()
        });
    }

    let one_two: HashSet<char> = rucksacks[0].intersection(&rucksacks[1]).cloned().collect();
    let two_three: HashSet<char> = one_two.intersection(&rucksacks[2]).cloned().collect();

    assert!(two_three.len() == 1);

    let value = two_three.iter().nth(0).unwrap();

    if value.is_lowercase() {
        *value as u32 - 'a' as u32 + 1
    } else if value.is_uppercase() {
        *value as u32 - 'A' as u32 + 27
    } else {
        panic!("Character not recognized");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let first = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let second = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let third = String::from("PmmdzqPrVvPwwTWBwg");

        let value = compute(first, second, third);

        assert_eq!(value, 18);
    }

    #[test]
    fn simple_second() {
        let first = String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let second = String::from("ttgJtRGJQctTZtZT");
        let third = String::from("CrZsJsPPZsGzwwsLwLmpwMDw");

        let value = compute(first, second, third);

        assert_eq!(value, 52);
    }
}
