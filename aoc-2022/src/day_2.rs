use crate::{Day, Puzzle};

mod common;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(2),
        String::from("Rock Paper Scissors"),
        Box::new(part_1::Solver),
        Box::new(part_2::Solver),
    )
}
