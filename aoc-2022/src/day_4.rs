use common::{Day, Puzzle, Solver};

mod both;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(4),
        String::from("Camp Cleanup"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
