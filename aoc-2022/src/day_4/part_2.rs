use std::io::BufRead;

use crate::day_4::both::*;

pub fn solve(input: &common::Input) -> i64 {
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

        overlap += if not_fully_overlaping(&first, &second) {
            0
        } else {
            1
        }
    }

    overlap as i64
}
