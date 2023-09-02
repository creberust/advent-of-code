use common::{Day, Puzzle};

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(3),
        String::from("Rucksack Reorganization"),
        Box::new(part_1::Solver),
        Box::new(part_2::Solver),
    )
}
