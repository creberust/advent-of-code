use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let lines = input.read().lines();

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_digit() {
        let zero = String::from("zad0azdazd");

        assert_eq!(solve(&Input::from(zero)), 0);
    }
}
