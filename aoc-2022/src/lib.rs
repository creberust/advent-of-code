use std::{collections::HashMap, path::Path};

use common::*;

mod day_1;

struct Event2022 {
    year: Year,
    puzzles: HashMap<Day, Puzzle<Box<dyn Fn(&Path)>>>,
}

impl Event2022 {
    fn new() -> Self {
        let year = Year(2022);
        let mut puzzles = HashMap::new();

        puzzles.insert(Day(1), day_1::puzzle());

        Self { year, puzzles }
    }
}

impl Event for Event2022 {
    fn year(&self) -> Year {
        self.year
    }

    fn solve(&self, day: Day, input: &Path, part: Part) {
        let puzzle = self.puzzles.get(&day);

        match puzzle {
            Some(puzzle) => {
                println!("\t{}", puzzle);
                puzzle.solve(input, part);
            }
            None => unimplemented!(),
        }
    }
}

pub fn event() -> Box<dyn Event> {
    Box::new(Event2022::new())
}
