use common::{Day, Puzzle, Solver};

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(5),
        String::from("Doesn't He Have Intern-Elves For This?"),
        Solver::new(part_1::solve),
        Solver::new(part_2::solve),
    )
}
