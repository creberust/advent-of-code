use std::path::PathBuf;

use cli::*;
mod cli;

use common::*;

use crate::aoc::AdventOfCode;

mod aoc;

fn main() {
    pretty_env_logger::init();

    let aoc = AdventOfCode::new();

    let args = get_cli_arg();

    // 1. Retrieve the year, or solve all years
    let year = match args.year {
        Some(year) => Year::from(year),
        None => return aoc.solve_all_years(),
    };
    log::debug!("Year: {}", year);

    // 1.a. Retrieve the event associated with the year.
    let event = aoc
        .get_event(year)
        .expect(&format!("Unimplemented Event for year: {}", year));

    // 2. Retrieve the day, or solve all day of the year.
    let day = match args.day {
        Some(day) => Day::from(day),
        None => return aoc.solve_all_days(event),
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
    aoc.solve_puzzle(&puzzle, input, part);
}
