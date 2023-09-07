use common::{Day, Puzzle, Solver};

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(4),
        String::from("The Ideal Stocking Stuffer"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
