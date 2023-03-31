use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use common::*;

mod day_1;
mod day_2;
mod day_3;

struct Event2022 {
    year: Year,
    puzzles: HashSet<Puzzle>,
}

impl Event2022 {
    fn new() -> Self {
        let year = Year(2022);
        let mut puzzles = HashSet::new();

        puzzles.insert(day_1::puzzle());
        puzzles.insert(day_2::puzzle());
        puzzles.insert(day_3::puzzle());

        Self { year, puzzles }
    }
}

impl Event for Event2022 {
    fn year(&self) -> Year {
        self.year
    }

    fn solve(&self, day: Day, input: &Path, part: Part) {
        let puzzle = self.puzzles.iter().find(|&p| p.day() == day);

        match puzzle {
            Some(puzzle) => {
                println!("|---{}", puzzle);
                puzzle.solve(&Input::from(input), part);
            }
            None => unimplemented!(),
        }
    }

    fn solve_all(&self) {
        for day in 0..=25 {
            let day = Day(day);

            let input = PathBuf::from(format!("input/aoc-{}/day_{}/input.txt", self.year, day));

            if !input.try_exists().unwrap() {
                continue;
            }

            self.solve(day, &input, Part::Both);
        }
    }
}

pub fn event() -> Box<dyn Event> {
    Box::new(Event2022::new())
}
