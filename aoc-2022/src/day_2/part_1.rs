use std::io::BufRead;

use crate::day_2::common::*;
use common::Input;

pub fn part_1(input: &Input) {
    let mut score: u32 = 0;

    for line in input.read().lines() {
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
