use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::day_2::common::*;

pub fn part_1(input: &Path) {
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    let mut score: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut it = line.chars();

        let opponent: Opponent = Opponent::from(it.next().expect("Invalid Opponent input"));
        let _ = it.next();
        let player: Player = Player::from(it.next().expect("Invalid Player input."));

        let round = fight(opponent, player);

        score += round;
    }

    println!("{}", score);
}

fn fight(opponent: Opponent, player: Player) -> u32 {
    match opponent {
        Opponent::Rock => match player {
            Player::Rock => 1 + DRAW,
            Player::Paper => 2 + WIN,
            Player::Scissors => 3 + LOSE,
        },
        Opponent::Paper => match player {
            Player::Rock => 1 + LOSE,
            Player::Paper => 2 + DRAW,
            Player::Scissors => 3 + WIN,
        },
        Opponent::Scissors => match player {
            Player::Rock => 1 + WIN,
            Player::Paper => 2 + LOSE,
            Player::Scissors => 3 + DRAW,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
