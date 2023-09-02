mod cli;
use cli::*;

use common::*;

fn main() {
    let args = get_cli_arg();

    let year = Year::from(args.year);
    let day = args.day;
    let part = match args.part {
        Some(part) => match part {
            PartValue::One => Part::One,
            PartValue::Two => Part::Two,
        },
        None => Part::Both,
    };
    let input = args.input;

    let event = match year {
        year if year.get() == 2022 => aoc_2022::event(),
        _ => unimplemented!(),
    };

    println!("{}", event);
    match day {
        Some(day) => {
            let input = input.unwrap();
            event.solve(Day::from(day), &input, part)
        }
        None => event.solve_all(),
    }
}
