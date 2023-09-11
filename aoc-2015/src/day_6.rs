use common::{Day, Puzzle, Solver};

mod grid;
mod instruction;
mod position;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(6),
        String::from("Probably a Fire Hazard"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
