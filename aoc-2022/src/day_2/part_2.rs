use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::day_2::common::*;

pub fn part_2(input: &Path) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut it = line.chars();

        let opponent: Opponent = Opponent::from(it.next().expect("Invalid Opponent input"));
        let _ = it.next();
        let round_status: RoundStatus =
            RoundStatus::from(it.next().expect("Invalid Player input."));

        let round = fight(opponent, round_status);

        score += round;
    }

    println!("{}", score);
}

fn fight(opponent: Opponent, round_status: RoundStatus) -> u32 {
    match opponent {
        Opponent::Rock => match round_status {
            RoundStatus::WIN => WIN + PAPER,
            RoundStatus::DRAW => DRAW + ROCK,
            RoundStatus::LOSE => LOSE + SCISSORS,
        },
        Opponent::Paper => match round_status {
            RoundStatus::WIN => WIN + SCISSORS,
            RoundStatus::DRAW => DRAW + PAPER,
            RoundStatus::LOSE => LOSE + ROCK,
        },
        Opponent::Scissors => match round_status {
            RoundStatus::WIN => WIN + ROCK,
            RoundStatus::DRAW => DRAW + SCISSORS,
            RoundStatus::LOSE => LOSE + PAPER,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
