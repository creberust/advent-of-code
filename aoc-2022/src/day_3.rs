use common::{Day, Input, Puzzle};

mod part_1;
use part_1::*;

mod part_2;
use part_2::*;

pub fn puzzle() -> Puzzle<Box<dyn Fn(&Input)>> {
    Puzzle::new(
        Day(3),
        String::from("Rucksack Reorganization"),
        Box::new(part_1),
        Box::new(part_2),
    )
}
