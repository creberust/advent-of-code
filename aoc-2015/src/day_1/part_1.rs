use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let mut result = 0;

    for line in input.read().lines() {
        let line = line.unwrap();

        for c in line.chars() {
            match c {
                '(' => result += 1,
                ')' => result -= 1,
                _ => continue,
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn zero() {
        check!("", 0);
        check!("(())", 0);
        check!("()()", 0);
    }

    #[test]
    fn three() {
        check!("(((", 3);
        check!("(()(()(", 3);
        check!("))(((((", 3);
    }

    #[test]
    fn minus_one() {
        check!("())", -1);
        check!("))(", -1);
    }

    #[test]
    fn minus_three() {
        check!(")))", -3);
        check!(")())())", -3);
    }
}
