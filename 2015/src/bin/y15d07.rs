use std::{collections::HashMap, str::FromStr};

use aoc::AoCError;

#[derive(Debug)]
enum Operand<'a> {
    Value(u16),
    Wire(&'a str),
}

impl<'a> Operand<'a> {
    fn new(s: &'a str) -> Self {
        if let Ok(value) = u16::from_str(s) {
            Operand::Value(value)
        } else {
            Operand::Wire(s)
        }
    }
}

#[derive(Debug)]
enum Operation<'a> {
    Assign(Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    LShift(Operand<'a>, Operand<'a>),
    RShift(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
}

impl<'a> Operation<'a> {
    fn new(s: &'a str) -> Result<Self, AoCError> {
        let mut parts = s.split_whitespace();

        match (parts.next(), parts.next(), parts.next()) {
            (Some(op), None, None) => Ok(Operation::Assign(Operand::new(op))),
            (Some("NOT"), Some(op), None) => Ok(Operation::Not(Operand::new(op))),
            (Some(op1), Some("AND"), Some(op2)) => {
                Ok(Operation::And(Operand::new(op1), Operand::new(op2)))
            }
            (Some(op1), Some("OR"), Some(op2)) => {
                Ok(Operation::Or(Operand::new(op1), Operand::new(op2)))
            }
            (Some(op1), Some("LSHIFT"), Some(op2)) => {
                Ok(Operation::LShift(Operand::new(op1), Operand::new(op2)))
            }
            (Some(op1), Some("RSHIFT"), Some(op2)) => {
                Ok(Operation::RShift(Operand::new(op1), Operand::new(op2)))
            }
            _ => Err(AoCError::BadInput),
        }
    }
}

fn find_signal<'a>(
    signal: &'a str,
    wiring: &HashMap<&'a str, Operation<'a>>,
    signals: &mut HashMap<&'a str, u16>,
) -> Result<u16, AoCError> {
    if let Some(signal) = signals.get(signal) {
        return Ok(*signal);
    }

    let value = match wiring.get(signal) {
        Some(Operation::Assign(op)) => match op {
            Operand::Value(value) => Ok(*value),
            Operand::Wire(wire) => find_signal(wire, wiring, signals),
        },
        Some(Operation::And(op1, op2)) => {
            let op1 = match op1 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            let op2 = match op2 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            Ok(op1 & op2)
        }
        Some(Operation::Or(op1, op2)) => {
            let op1 = match op1 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            let op2 = match op2 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            Ok(op1 | op2)
        }
        Some(Operation::LShift(op1, op2)) => {
            let op1 = match op1 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            let op2 = match op2 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            Ok(op1 << op2)
        }
        Some(Operation::RShift(op1, op2)) => {
            let op1 = match op1 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            let op2 = match op2 {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            Ok(op1 >> op2)
        }
        Some(Operation::Not(op)) => {
            let op = match op {
                Operand::Value(value) => *value,
                Operand::Wire(wire) => find_signal(wire, wiring, signals)?,
            };

            Ok(!op)
        }
        _ => Err(AoCError::BadInput),
    };

    if let Ok(value) = value {
        signals.insert(signal.clone(), value);
        Ok(value)
    } else {
        value
    }
}

fn solve_task(input: &str, signal: &str) -> (u16, u16) {
    let wiring: HashMap<&str, Operation> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(" -> ");
            let operation = Operation::new(parts.next().unwrap()).unwrap();
            let wire = parts.next().unwrap();

            (wire, operation)
        })
        .collect();

    let mut signals: HashMap<&str, u16> = HashMap::new();

    let task1 = find_signal(signal, &wiring, &mut signals).expect("Bad input");

    signals.clear();
    signals.insert("b", task1);
    let task2 = find_signal(signal, &wiring, &mut signals).expect("Bad input");

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2015,
        7,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input, "a");

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d07 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

        let (example1, _) = solve_task(example_input, "d");
        assert_eq!(example1, 72);

        let (example1, _) = solve_task(example_input, "e");
        assert_eq!(example1, 507);

        let (example1, _) = solve_task(example_input, "f");
        assert_eq!(example1, 492);

        let (example1, _) = solve_task(example_input, "g");
        assert_eq!(example1, 114);

        let (example1, _) = solve_task(example_input, "h");
        assert_eq!(example1, 65412);

        let (example1, _) = solve_task(example_input, "i");
        assert_eq!(example1, 65079);

        let (example1, _) = solve_task(example_input, "x");
        assert_eq!(example1, 123);

        let (example1, _) = solve_task(example_input, "y");
        assert_eq!(example1, 456);
    }
}
