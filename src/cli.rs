use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The advent of code event for the given year
    pub year: u32,

    /// The day from the selected advent of code event
    pub day: u8,
}

/// Return the parsed arguments from the Command-Line Interface.
pub fn get_cli_arg() -> Cli {
    Cli::parse()
}
