use common::{Day, Puzzle, Solver};

mod both;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(2),
        String::from("Rock Paper Scissors"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
