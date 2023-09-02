use common::Solution;

use std::io::BufRead;

use crate::day_4::both::*;

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &common::Input) -> u32 {
        let mut overlap: u32 = 0;

        for line in input.read().lines() {
            let l = line.unwrap();

            let elements: Vec<&str> = l.split_terminator(['-', ',']).collect();
            let first = Sections::new(
                elements[0].to_string().parse().unwrap(),
                elements[1].to_string().parse().unwrap(),
            );
            let second = Sections::new(
                elements[2].to_string().parse().unwrap(),
                elements[3].to_string().parse().unwrap(),
            );

            overlap += if full_overlaping(&first, &second) {
                1
            } else {
                0
            };
        }

        overlap
    }
}
