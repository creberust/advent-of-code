use std::{collections::HashSet, io::BufRead};

use common::Input;

use super::both::{Direction, Position};

pub fn solve(input: &Input) -> i64 {
    let mut position_visited = HashSet::<Position>::new();
    let mut santa_pos = Position::new(0, 0);
    let mut robot_pos = Position::new(0, 0);

    // Add the starting location as visited
    position_visited.insert(santa_pos);

    for line in input.read().lines() {
        let line = line.unwrap();

        let mut dir = line.chars().map(Direction::from);

        loop {
            let santa_dir = dir.next();
            let robot_dir = dir.next();

            if santa_dir.is_none() || robot_dir.is_none() {
                break;
            }

            let santa_dir = santa_dir.unwrap();
            let robot_dir = robot_dir.unwrap();

            santa_pos = update_pos(santa_pos, santa_dir);
            robot_pos = update_pos(robot_pos, robot_dir);

            position_visited.insert(santa_pos);
            position_visited.insert(robot_pos);
        }
    }

    position_visited.len() as i64
}

/// Return the [Position] for `pos` according to the given `dir`.
fn update_pos(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::North => Position::new(pos.x, pos.y + 1),
        Direction::South => Position::new(pos.x, pos.y - 1),
        Direction::East => Position::new(pos.x + 1, pos.y),
        Direction::West => Position::new(pos.x - 1, pos.y),
    }
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!("^v", 3);
        check!("^>v<", 3);
        check!("^v^v^v^v^v", 11);
    }
}
