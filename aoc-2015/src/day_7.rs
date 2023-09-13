use common::{Day, Puzzle, Solver};

mod both;
mod part_1;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(7),
        String::from("Some Assembly Required"),
        Solver::new(part_1::solve),
        Solver::new(part_1::solve), // Part 2 is the same as Part 1 by with different input (input.txt and input_2.txt)
    )
}
