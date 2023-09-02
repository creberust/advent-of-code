mod cli;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use cli::*;

use common::*;

fn main() {
    pretty_env_logger::init();

    let event_per_year: HashMap<Year, Event> = [
        (Year::from(2015), aoc_2015::event()),
        (Year::from(2022), aoc_2022::event()),
    ]
    .into();

    let args = get_cli_arg();

    // 1. Retrieve the year, or solve all years
    let year = match args.year {
        Some(year) => Year::from(year),
        None => return solve_all_years(),
    };
    log::debug!("Year: {}", year);

    // 1.a. Retrieve the event associated with the year.
    let event = event_per_year
        .get(&year)
        .expect(&format!("Unimplemented Event for year: {}", year));

    // 2. Retrieve the day, or solve all day of the year.
    let day = match args.day {
        Some(day) => Day::from(day),
        None => return solve_all_days(event),
    };
    log::debug!("Day: {}", day);

    // 2.a. Retrieve the Puzzle for the specified day.
    let puzzle = event
        .puzzle(day)
        .expect(&format!("Unimplemented Puzzle for day: {}", day));

    // 3. Retrieve the specified part, or solve both parts.
    let part = match args.part {
        Some(part) => match part {
            PartValue::One => Part::One,
            PartValue::Two => Part::Two,
        },
        None => Part::Both,
    };
    log::debug!("Part: {}", part);

    // 4. Retrieve the input or take the default one.
    let input = match args.input {
        Some(input) => input,
        None => PathBuf::from(format!("input/{}/{}/input.txt", year, day)),
    };
    log::debug!("Input: {:?}", input);

    println!("|--- {}", puzzle);
    solve_puzzle(&puzzle, input, part);
}

fn solve_all_years() {
    for event in [aoc_2015::event(), aoc_2022::event()] {
        println!("{}", event);

        solve_all_days(&event)
    }
}

fn solve_all_days(event: &Event) {
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
        solve_puzzle(event.puzzle(day).unwrap(), &input, Part::Both);
    }
}

fn solve_puzzle(puzzle: &Puzzle, input: impl AsRef<Path>, part: Part) {
    match part {
        Part::One => {
            println!("   \\--- Part One");
            let result = solve(puzzle, input, Part::One);
            println!("      \\--- Result: {}", result);
        }
        Part::Two => {
            println!("   \\--- Part One");
            let result = solve(puzzle, input, Part::Two);
            println!("      \\--- Result: {}", result);
        }
        Part::Both => {
            solve_puzzle(puzzle, input.as_ref(), Part::One);
            solve_puzzle(puzzle, input.as_ref(), Part::Two)
        }
    }
}

fn solve(puzzle: &Puzzle, input: impl AsRef<Path>, part: Part) -> u32 {
    match part {
        Part::One => puzzle.solve_one(&Input::File(input.as_ref().to_path_buf())),
        Part::Two => puzzle.solve_two(&Input::File(input.as_ref().to_path_buf())),
        _ => unreachable!(),
    }
}
