use std::{collections::HashSet, io::BufRead};

use common::{Input, Solution};

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) {
        let mut priority: u32 = 0;

        for line in input.read().lines() {
            let line = line.unwrap();

            priority += rucksacks_priority(line);
        }

        println!("{}", priority);
    }
}

pub struct Rucksack {
    count: usize,
    mid: usize,
    compartment: (HashSet<char>, HashSet<char>),
}

impl Rucksack {
    fn new(size: usize) -> Self {
        Rucksack {
            count: 0,
            mid: size / 2,
            compartment: (HashSet::new(), HashSet::new()),
        }
    }

    fn fill(&mut self, c: char) {
        if self.count < self.mid {
            self.compartment.0.insert(c);
        } else {
            self.compartment.1.insert(c);
        }

        self.count += 1;
    }

    fn get_priority(&self) -> u32 {
        let dup: HashSet<char> = self
            .compartment
            .0
            .intersection(&self.compartment.1)
            .cloned()
            .collect();

        assert!(dup.len() == 1);

        let value = dup.iter().nth(0).unwrap();

        if value.is_lowercase() {
            *value as u32 - 'a' as u32 + 1
        } else if value.is_uppercase() {
            *value as u32 - 'A' as u32 + 27
        } else {
            panic!("Character not recognized");
        }
    }
}

fn rucksacks_priority(input: String) -> u32 {
    assert!(input.len() % 2 == 0);

    let mut rucksack = Rucksack::new(input.len());

    input.chars().into_iter().for_each(|c| rucksack.fill(c));

    rucksack.get_priority()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_two_letter_a() {
        let input = String::from("aa");

        assert_eq!(rucksacks_priority(input), 1);
    }

    #[test]
    fn size_four_letter_a_uppercase() {
        let input = String::from("lAdA");

        assert_eq!(rucksacks_priority(input), 27);
    }

    #[test]
    fn size_six_letter_v() {
        let input = String::from("lBvdvv");

        assert_eq!(rucksacks_priority(input), 22);
    }

    #[test]
    fn size_eight_letter_z_uppercase() {
        let input = String::from("ZZBBdvAZ");

        assert_eq!(rucksacks_priority(input), 52);
    }
}
