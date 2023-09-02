use std::io::BufRead;

use crate::day_2::both::*;
use common::{Input, Solution};

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) -> u32 {
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

        score
    }
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
    fn player_rock() {
        // Given
        let player = Player::Rock;
        let opponents = vec![Opponent::Rock, Opponent::Paper, Opponent::Scissors];
        let expected = vec![ROCK + DRAW, ROCK + LOSE, ROCK + WIN];

        // When
        for i in 0..opponents.len() {
            let actual = fight(opponents[i], player);

            // Then
            assert_eq!(actual, expected[i]);
        }
    }

    #[test]
    fn player_paper() {
        // Given
        let player = Player::Paper;
        let opponents = vec![Opponent::Rock, Opponent::Paper, Opponent::Scissors];
        let expected = vec![PAPER + WIN, PAPER + DRAW, PAPER + LOSE];

        // When
        for i in 0..opponents.len() {
            let actual = fight(opponents[i], player);

            // Then
            assert_eq!(actual, expected[i]);
        }
    }

    #[test]
    fn player_scissors() {
        // Given
        let player = Player::Scissors;
        let opponents = vec![Opponent::Rock, Opponent::Paper, Opponent::Scissors];
        let expected = vec![SCISSORS + LOSE, SCISSORS + WIN, SCISSORS + DRAW];

        // When
        for i in 0..opponents.len() {
            let actual = fight(opponents[i], player);

            // Then
            assert_eq!(actual, expected[i]);
        }
    }
}
