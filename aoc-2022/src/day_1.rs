use std::path::Path;

use common::{Day, Puzzle};

mod part_1;
use part_1::*;

mod part_2;
use part_2::*;

pub fn puzzle() -> Puzzle<Box<dyn Fn(&Path)>> {
    Puzzle::new(
        Day(1),
        String::from("Calorie Counting"),
        Box::new(part_1),
        Box::new(part_2),
    )
}
