use std::io::BufRead;

use common::{Input, Solution};
use md5::{Digest, Md5};

pub struct Solver;

impl Solution for Solver {
    fn solve(&self, input: &Input) -> i64 {
        let input = input.read().lines().next().unwrap().unwrap();

        let mut number: u32 = 1;

        loop {
            let mut hasher = Md5::new();

            let secret_key = format!("{}{}", input, number);

            hasher.update(secret_key.as_bytes());

            let result = hasher.finalize();

            if result[0] == 0 && result[1] == 0 && result[2] == 0 {
                break;
            }

            number += 1;
        }

        number as i64
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple() {}
}
