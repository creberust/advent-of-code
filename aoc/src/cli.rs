use std::path::PathBuf;

use clap::{value_parser, Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The advent of code event for the given year.
    ///
    /// By default, every year will be solved.
    #[arg(value_parser = value_parser!(u16).range(2015..=2023))]
    pub year: Option<u16>,

    /// The day from the selected advent of code event.
    ///
    /// By default, every puzzles will be solved.
    #[arg(value_parser = value_parser!(u8).range(1..=25))]
    pub day: Option<u8>,

    /// The path to the input file.
    pub input: Option<PathBuf>,

    /// The part of the puzzle you want to execute.
    ///
    /// By default, both parts are solved one after the other.
    pub part: Option<PartValue>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum PartValue {
    /// Only solve puzzle part one.
    One,
    /// Only solve puzzle part two.
    Two,
}

/// Return the parsed arguments from the Command-Line Interface.
pub fn get_cli_arg() -> Cli {
    Cli::parse()
}
