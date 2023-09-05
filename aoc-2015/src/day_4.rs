use common::{Day, Puzzle};

mod both;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(4),
        String::from("The Ideal Stocking Stuffer"),
        Box::new(part_1::Solver),
        Box::new(part_2::Solver),
    )
}
