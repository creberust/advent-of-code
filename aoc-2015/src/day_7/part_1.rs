use std::{collections::HashMap, io::BufRead};

use common::Input;

use super::both::{Expr, Operator, Wire};

pub fn solve(input: &Input) -> i64 {
    let mut wires = HashMap::<Wire, Expr>::new();

    for line in input.read().lines().map(|line| line.unwrap()) {
        // Forms:
        // 1. <wire|value> -> <wire>
        // 2. <op> <wire|value> -> <wire>
        // 3. <wire|value> <op> <wire|value> -> <wire>

        let values: Vec<&str> = line.split(' ').collect();

        if values.len() == 3 {
            // 1. This is the value assigned to a Wire.
            let input = match values[0].parse::<u16>() {
                Ok(value) => Expr::Numeral(value),
                Err(_) => Expr::Identifier(Wire::from(values[0])),
            };
            let output = Wire::from(values[2]);

            wires.insert(output, input);
        } else if values.len() == 4 {
            // 2. This is the NOT operator with the Wire assigned to a Wire.
            let op = Operator::from(values[0]);
            assert_eq!(op, Operator::NOT);

            let input = match values[1].parse::<u16>() {
                Ok(value) => Expr::Numeral(value),
                Err(_) => Expr::Identifier(Wire::from(values[1])),
            };
            let output = Wire::from(values[3]);

            let expr = Expr::Operation(Box::new(input), Box::new(Expr::Numeral(0)), op);

            wires.insert(output, expr);
        } else if values.len() == 5 {
            // 3. This is the operation between 2 wires/values assigned to a Wire.
            let input_a = match values[0].parse::<u16>() {
                Ok(value) => Expr::Numeral(value),
                Err(_) => Expr::Identifier(Wire::from(values[0])),
            };
            let input_b = match values[2].parse::<u16>() {
                Ok(value) => Expr::Numeral(value),
                Err(_) => Expr::Identifier(Wire::from(values[2])),
            };

            let op = Operator::from(values[1]);

            let output = Wire::from(values[4]);

            let expr = Expr::Operation(Box::new(input_a), Box::new(input_b), op);

            wires.insert(output, expr);
        }
    }

    print(&wires);

    compute(
        &wires,
        wires.get(&Wire::new("a")).unwrap(),
        &mut HashMap::new(),
    ) as i64
}

fn print(wires: &HashMap<Wire, Expr>) {
    for (key, value) in wires {
        println!("{} -> {}", value, key);
    }
}

fn compute(wires: &HashMap<Wire, Expr>, expr: &Expr, cache: &mut HashMap<Wire, u16>) -> u16 {
    log::trace!("exec: {}", expr);

    match expr {
        Expr::Numeral(value) => *value,
        Expr::Identifier(wire) => {
            if let Some(value) = cache.get(&wire) {
                return *value;
            }

            let value = compute(wires, wires.get(&wire).unwrap(), cache);

            cache.insert(wire.clone(), value);

            value
        }
        Expr::Operation(input_a, input_b, op) => {
            let (value_a, value_b) = match op {
                Operator::NOT => (compute(wires, &input_a, cache), 0),
                _ => (
                    compute(wires, &input_a, cache),
                    compute(wires, &input_b, cache),
                ),
            };

            match op {
                Operator::NOT => !value_a,
                Operator::AND => value_a & value_b,
                Operator::OR => value_a | value_b,
                Operator::LSHIFT => value_a << value_b,
                Operator::RSHIFT => value_a >> value_b,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use common::{check, Checker};

    use super::*;

    const CHECKER: Checker = Checker::new(solve);

    #[test]
    fn simple() {
        check!("42 -> a", 42);
        check!("42 -> b\nNOT b -> a", !(42 as u16) as i64);
        check!("42 -> b\nNOT b -> a", !(42 as u16) as i64);
    }
}
