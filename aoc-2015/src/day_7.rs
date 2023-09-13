use common::{Day, Puzzle, Solver};

mod both;
mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(7),
        String::from("Some Assembly Required"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
