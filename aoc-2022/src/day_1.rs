use common::{Day, Puzzle, Solver};

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(1),
        String::from("Calorie Counting"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
