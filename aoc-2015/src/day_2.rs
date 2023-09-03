use common::{Day, Puzzle};

mod both;

mod part_1;
mod part_2;

pub fn puzzle() -> Puzzle {
    Puzzle::new(
        Day::from(2),
        String::from("I Was Told There Would Be No Math"),
        Box::new(part_1::Solver),
        Box::new(part_2::Solver),
    )
}
