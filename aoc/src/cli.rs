use clap::{value_parser, Parser};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The advent of code event for the given year
    #[arg(value_parser = value_parser!(u16).range(2015..=2022))]
    pub year: u16,

    /// The day from the selected advent of code event
    #[arg(value_parser = value_parser!(u8).range(1..=25))]
    pub day: u8,
}

/// Return the parsed arguments from the Command-Line Interface.
pub fn get_cli_arg() -> Cli {
    Cli::parse()
}
