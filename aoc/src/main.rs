mod cli;
use cli::*;

use common::*;

fn main() {
    let args = get_cli_arg();

    let year = Year(args.year);
    let day = Day(args.day);
    let part = match args.part {
        Some(part) => match part {
            PartValue::One => Part::One,
            PartValue::Two => Part::Two,
        },
        None => Part::Both,
    };
    let input = args.input.as_path();

    let event = match year {
        Year(2022) => aoc_2022::event(),
        _ => unimplemented!(),
    };

    println!("{}", event);
    event.solve(day, input, part);
}
