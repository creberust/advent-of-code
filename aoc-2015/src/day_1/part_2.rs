use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let mut result = 0;
    let mut position = 1;

    for line in input.read().lines() {
        let line = line.unwrap();

        for c in line.chars() {
            match c {
                '(' => result += 1,
                ')' => result -= 1,
                _ => continue,
            }

            if result < 0 {
                break;
            }

            position += 1;
        }
    }

    position
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn one() {
        check!(")", 1);
    }

    #[test]
    fn five() {
        check!("()())", 5);
    }
}
