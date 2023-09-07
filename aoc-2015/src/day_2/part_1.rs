use std::io::BufRead;

use common::Input;

use super::both::Present;

pub fn solve(input: &Input) -> i64 {
    let mut wrapping_paper = 0;

    for line in input.read().lines() {
        let line = line.unwrap();

        let present = Present::from(line.as_str());

        wrapping_paper += present.surface_area() + present.smallest_area();
    }

    wrapping_paper as i64
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!("2x3x4", 58);
        check!("1x1x10", 43);
    }
}
