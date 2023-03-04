use common::{Day, Puzzle};

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

    fn solve(&self, part: common::Part) {}
}

pub fn puzzle() -> Box<dyn Puzzle> {
    Box::new(Day1::new())
}
