//! Implementation of Advent of Code.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use common::{Day, Event, Input, Part, Puzzle, Year};

pub struct AdventOfCode {
    /// All the events per year.
    events: HashMap<Year, Event>,
}

impl AdventOfCode {
    /// Create a new Advent of Code with all the implemented years.
    pub fn new() -> Self {
        Self {
            events: [
                (Year::from(2015), aoc_2015::event()),
                (Year::from(2022), aoc_2022::event()),
            ]
            .into(),
        }
    }

    /// Get the event associated to the given `year`.
    pub fn get_event(&self, year: Year) -> Option<&Event> {
        self.events.get(&year)
    }

    /// Solve all of the Advent of Code events.
    pub fn solve_all_years(&self) {
        for event in self.events.values() {
            println!("{}", event);

            self.solve_all_days(&event)
        }
    }

    /// Solve all of the days in the specified `event`.
    pub fn solve_all_days(&self, event: &Event) {
        for day in 1..=25 {
            let day = Day::from(day);

            let input = PathBuf::from(format!("input/{}/{}/input.txt", event.year(), day));

            if !input.try_exists().unwrap() {
                continue;
            }

            let puzzle = event
                .puzzle(day)
                .expect(&format!("Unimplemented Puzzle for day: {}", day));

            println!("|--- {}", puzzle);
            self.solve_puzzle(event.puzzle(day).unwrap(), &input, Part::Both);
        }
    }

    /// Solve the given `puzzle` `part` with its `input`.
    pub fn solve_puzzle(&self, puzzle: &Puzzle, input: impl AsRef<Path>, part: Part) {
        match part {
            Part::One => {
                println!("   \\--- Part One");
                let result = self.solve(puzzle, input, Part::One);
                println!("      \\--- Result: {}", result);
            }
            Part::Two => {
                println!("   \\--- Part Two");
                let result = self.solve(puzzle, input, Part::Two);
                println!("      \\--- Result: {}", result);
            }
            Part::Both => {
                self.solve_puzzle(puzzle, input.as_ref(), Part::One);
                self.solve_puzzle(puzzle, input.as_ref(), Part::Two)
            }
        }
    }

    fn solve(&self, puzzle: &Puzzle, input: impl AsRef<Path>, part: Part) -> u32 {
        match part {
            Part::One => puzzle.solve_one(&Input::File(input.as_ref().to_path_buf())),
            Part::Two => puzzle.solve_two(&Input::File(input.as_ref().to_path_buf())),
            _ => unreachable!(),
        }
    }
}
