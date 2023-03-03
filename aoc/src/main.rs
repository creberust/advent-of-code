mod cli;
use cli::*;

use common::*;

fn main() {
    let args = get_cli_arg();

    let year = Year(args.year);
    let day = Day(args.day);

    let event = match year {
        Year(2022) => aoc_2022::event(),
        _ => unimplemented!(),
    };

    event.solve(day, Part::One);
}
