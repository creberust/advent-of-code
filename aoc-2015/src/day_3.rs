use common::{Day, Puzzle, Solver};

mod both;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(3),
        String::from("Perfectly Spherical Houses in a Vacuum"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
