use common::{Day, Puzzle, Solver};

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(3),
        String::from("Rucksack Reorganization"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
