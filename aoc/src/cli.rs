use clap::{value_parser, Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The advent of code event for the given year
    #[arg(value_parser = value_parser!(u16).range(2015..=2022))]
    pub year: u16,

    /// The day from the selected advent of code event
    #[arg(value_parser = value_parser!(u8).range(1..=25))]
    pub day: u8,

    /// The part of the puzzle you want to execute
    ///
    /// By default, both parts are solved one after the other
    pub part: Option<PartValue>,
}

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum)]
pub enum PartValue {
    /// Only solve puzzle part one
    One,
    /// Only solve puzzle part two
    Two,
}

/// Return the parsed arguments from the Command-Line Interface.
pub fn get_cli_arg() -> Cli {
    Cli::parse()
}
