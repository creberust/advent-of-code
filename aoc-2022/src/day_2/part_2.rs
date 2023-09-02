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
            let round_status: RoundStatus =
                RoundStatus::from(it.next().expect("Invalid Player input."));

            let round = fight(opponent, round_status);

            score += round;
        }

        score
    }
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
    fn opponent_rock() {
        // Given
        let opponent = Opponent::Rock;
        let rounds = vec![RoundStatus::WIN, RoundStatus::DRAW, RoundStatus::LOSE];
        let expected = vec![PAPER + WIN, ROCK + DRAW, SCISSORS + LOSE];

        for i in 0..rounds.len() {
            // When
            let actual = fight(opponent, rounds[i]);

            // Then
            assert_eq!(actual, expected[i]);
        }
    }

    #[test]
    fn opponent_paper() {
        // Given
        let opponent = Opponent::Paper;
        let rounds = vec![RoundStatus::WIN, RoundStatus::DRAW, RoundStatus::LOSE];
        let expected = vec![SCISSORS + WIN, PAPER + DRAW, ROCK + LOSE];

        for i in 0..rounds.len() {
            // When
            let actual = fight(opponent, rounds[i]);

            // Then
            assert_eq!(actual, expected[i]);
        }
    }

    #[test]
    fn opponent_scissors() {
        // Given
        let opponent = Opponent::Scissors;
        let rounds = vec![RoundStatus::WIN, RoundStatus::DRAW, RoundStatus::LOSE];
        let expected = vec![ROCK + WIN, SCISSORS + DRAW, PAPER + LOSE];

        for i in 0..rounds.len() {
            // When
            let actual = fight(opponent, rounds[i]);

            // Then
            assert_eq!(actual, expected[i]);
        }
    }
}
