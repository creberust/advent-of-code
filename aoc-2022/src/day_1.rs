use common::{Day, Part, Puzzle};

mod part_1;
use part_1::*;

mod part_2;
use part_2::*;

struct Day1 {
    day: Day,
    name: String,
}

impl Day1 {
    fn new() -> Self {
        let day = Day(1);
        let name = String::from("Calorie Counting");

        Self { day, name }
    }
}

impl Puzzle for Day1 {
    fn day(&self) -> Day {
        self.day
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn solve(&self, part: Part) {
        match part {
            Part::One => part_1(),
            Part::Two => part_2(),
            Part::Both => {
                self.solve(Part::One);
                self.solve(Part::Two);
            }
        }
    }
}

pub fn puzzle() -> Box<dyn Puzzle> {
    Box::new(Day1::new())
}
