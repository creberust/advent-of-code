use std::path::Path;

use crate::{Day, Puzzle};

mod common;

mod part_1;
use part_1::*;

mod part_2;
use part_2::*;

pub fn puzzle() -> Puzzle<Box<dyn Fn(&Path)>> {
    Puzzle::new(
        Day(2),
        String::from("Rock Paper Scissors"),
        Box::new(part_1),
        Box::new(part_2),
    )
}
