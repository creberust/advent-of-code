use std::io::BufRead;

use common::Input;

use super::both::Present;

pub fn solve(input: &Input) -> i64 {
    let mut feet_of_ribon = 0;

    for line in input.read().lines() {
        let line = line.unwrap();

        let present = Present::from(line.as_str());

        feet_of_ribon += present.volume() + present.smallest_perimeter();
    }

    feet_of_ribon as i64
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!("2x3x4", 34);
        check!("1x1x10", 14);
    }
}
