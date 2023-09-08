use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let mut nice = 0;

    for line in input.read().lines().map(|l| l.unwrap()) {
        // Checks:
        // 1. Contains at least 3 voyels: (aeiuo).
        // 2. Contains at least 1 letter that appears twice in a row.
        // 3. Does not contain the strings: ab, cd, pq, xy.

        let mut voyels = 0;
        let mut previous_letter = char::default();
        let invalid_pairs = vec![('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];

        let mut twice_letters = false;
        let mut invalid_string = false;

        for c in line.chars() {
            voyels += if is_voyel(c) { 1 } else { 0 };

            if previous_letter == c {
                twice_letters = true;
            }

            if invalid_pairs.contains(&(previous_letter, c)) {
                invalid_string = true;

                break;
            }

            previous_letter = c;
        }

        if voyels >= 3 && twice_letters && !invalid_string {
            nice += 1;
        }
    }

    nice
}

/// Check if a char `c` is a voyel.
fn is_voyel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!("ugknbfddgicrmopn", 1);
        check!("aaa", 1);
        check!("jchzalrnumimnmhp", 0);
        check!("haegwjzuvuyypxyu", 0);
        check!("dvszwmarrgswjxmb", 0);
    }
}
