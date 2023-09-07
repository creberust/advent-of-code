use std::{collections::HashSet, io::BufRead};

use common::Input;

use super::both::{Direction, Position};

pub fn solve(input: &Input) -> i64 {
    let mut position_visited = HashSet::<Position>::new();
    let mut pos = Position::new(0, 0);

    // Add the starting location as visited
    position_visited.insert(pos);

    for line in input.read().lines() {
        let line = line.unwrap();

        for dir in line.chars().map(Direction::from) {
            pos = match dir {
                Direction::North => Position::new(pos.x, pos.y + 1),
                Direction::South => Position::new(pos.x, pos.y - 1),
                Direction::East => Position::new(pos.x + 1, pos.y),
                Direction::West => Position::new(pos.x - 1, pos.y),
            };

            position_visited.insert(pos);
        }
    }

    position_visited.len() as i64
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!(">", 2);
        check!("^>v<", 4);
        check!("^v^v^v^v^v", 2);
    }
}
