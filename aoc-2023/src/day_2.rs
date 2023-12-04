use common::{Day, Puzzle, Solver};

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(2),
        String::from("Cube Conundrum"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
