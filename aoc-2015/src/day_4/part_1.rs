use md5::{Digest, Md5};
use std::io::BufRead;

use common::Input;

pub fn solve(input: &Input) -> i64 {
    let input = input.read().lines().next().unwrap().unwrap();

    let mut number: u32 = 1;

    loop {
        let mut hasher = Md5::new();

        let secret_key = format!("{}{}", input, number);

        hasher.update(secret_key.as_bytes());

        let result = hasher.finalize();

        if result[0] == 0 && result[1] == 0 && (result[2] & (15 << 4)) == 0 {
            break;
        }

        number += 1;
    }

    number as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: impl AsRef<str>, expected: i64) {
        // Given
        let input = Input::Text(String::from(input.as_ref()));

        // When
        let result = solve(&input);

        // Then
        assert_eq!(result, expected);
    }

    #[test]
    fn simple() {
        check("abcdef", 609043);
        check("pqrstuv", 1048970);
    }
}
