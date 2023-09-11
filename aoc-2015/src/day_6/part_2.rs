mod grid;

use std::io::BufRead;

use common::Input;

use self::grid::Grid;

use super::{instruction::Instruction, position::Position};

pub fn solve(input: &Input) -> i64 {
    let mut instructions = Vec::<(Instruction, (Position, Position))>::new();
    let mut grid = Grid::new();

    for line in input.read().lines().map(|line| line.unwrap()) {
        // Each line has a pattern:
        // {instruction} {position} through {position}

        // 1. Parse the Instruction.
        let instruction = if line.starts_with("turn on") {
            Instruction::TurnOn
        } else if line.starts_with("turn off") {
            Instruction::TurnOff
        } else if line.starts_with("toggle") {
            Instruction::Toggle
        } else {
            log::error!("invalid instruction: {}", line);
            continue;
        };

        let line = match instruction {
            Instruction::TurnOn => &line[8..],
            Instruction::TurnOff => &line[9..],
            Instruction::Toggle => &line[7..],
        };

        let positions: Vec<&str> = line.split(' ').collect();
        if positions.len() != 3 {
            log::error!("invalid range: {}", line);
        }

        // 2. Parse the first position.
        let start = Position::from(*positions.get(0).unwrap());

        // 3. Parse the second position.
        let end = Position::from(*positions.get(2).unwrap());

        instructions.push((instruction, (start, end)));
    }

    instructions
        .iter()
        .for_each(|(instr, range)| execute(&mut grid, *instr, *range));

    grid.total_brightness() as i64
}

fn execute(grid: &mut Grid, instr: Instruction, range: (Position, Position)) {
    log::debug!("{} in range [{}, {}]", instr, range.0, range.1);

    let (start, end) = range;

    for x in start.x()..=end.x() {
        for y in start.y()..=end.y() {
            match instr {
                Instruction::TurnOn => grid.increase(Position::new(x, y)),
                Instruction::TurnOff => grid.decrease(Position::new(x, y)),
                Instruction::Toggle => {
                    grid.increase(Position::new(x, y));
                    grid.increase(Position::new(x, y))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use common::{check, Checker};

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!("turn on 0,0 through 0,0", 1);
        check!("toggle 0,0 through 999,999", 2000000);
    }
}
