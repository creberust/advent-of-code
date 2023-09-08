use std::{collections::HashMap, io::BufRead};

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let mut nice = 0;

    for line in input.read().lines().map(|l| l.unwrap()) {
        // Checks:
        // 1. Contains a pair of any two letters that appears at least twice without overlaping.
        // 2. Contains at least one letter wich repeats with exactly one letter between them.

        if has_repeats(&line) && has_valid_pair(&line) {
            nice += 1;
        }
    }

    nice
}

fn has_repeats(s: impl AsRef<str>) -> bool {
    let mut it = s.as_ref().chars().peekable();

    let mut previous = char::default();

    while let Some(c) = it.next() {
        let next = it.peek().map(|c| *c);

        // Check for 2.
        if Some(previous) == next {
            return true;
        }

        previous = c;
    }

    false
}

fn has_valid_pair(s: impl AsRef<str>) -> bool {
    // Construct an HashMap containing the pairs with the index of the start of the pair.
    let mut pairs = HashMap::<(char, char), Vec<usize>>::new();

    let mut previous = s.as_ref().chars().nth(0).unwrap();

    for (index, c) in s.as_ref().chars().skip(1).enumerate() {
        match pairs.get_mut(&(previous, c)) {
            Some(indexes) => indexes.push(index),
            None => {
                pairs.insert((previous, c), vec![index]);
                ()
            }
        };

        previous = c;
    }

    for (chars, indexes) in pairs.iter().filter(|(_, indexes)| indexes.len() > 1) {
        // Valid case is if the pair has different char because it cannot overlap
        if chars.0 != chars.1 {
            return true;
        }

        let mut it_second = indexes.iter().skip(1);

        let first = indexes.get(0).unwrap();

        while let Some(second) = it_second.next() {
            if second - first != 1 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;

    use common::{check, Checker};

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn repeats() {
        assert!(!has_repeats(""));
        assert!(!has_repeats("a"));
        assert!(!has_repeats("aa"));
        assert!(!has_repeats("abc"));

        assert!(has_repeats("aaa"));
        assert!(has_repeats("aoa"));
    }

    #[test]
    fn pairs() {
        assert!(!has_valid_pair("aaa"));

        assert!(has_valid_pair("aaaa"));
    }

    #[test]
    fn simple() {
        check!("abcdefghijqlmnop", 0);
        check!("abcdefghijqlmnmp", 0);
        check!("abcdefcdijqlmnmp", 1);
        check!("abcdefcdijqlmnmp", 1);
        check!("abcdefcdijqlmnmaboakdozab", 1);

        check!("ppp", 0);
        check!("qjhvhtzxzqqjkmpb", 1);
        check!("xxyxx", 1);
        check!("uurcxstgmygtbstg", 0);
        check!("ieodomkazucvgmuy", 0);
    }
}
