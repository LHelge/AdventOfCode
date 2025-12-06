const YEAR: u16 = 2024;
const DAY: u8 = 7;
use aoc::utils::*;
use aoc::*;
use rayon::prelude::*;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Operator {
    #[default]
    Add,
    Multiply,
    Concatenate,
}

#[derive(Debug)]
struct Equation {
    test: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn test(&self, operators: &[Operator]) -> bool {
        if self.operands.len() != operators.len() + 1 {
            return false;
        }

        let mut operands = self.operands.iter();
        let first = *operands.next().unwrap();

        let result = operands
            .zip(operators.iter())
            .fold(first, |acc, (operand, operator)| match operator {
                Operator::Add => acc + operand,
                Operator::Multiply => acc * operand,
                Operator::Concatenate => {
                    let mut str = acc.to_string();
                    str.push_str(&operand.to_string());
                    str.parse().unwrap()
                }
            });

        result == self.test
    }
}

impl FromStr for Equation {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let (test, operands) = s.split_once(": ").ok_or(AoCError::BadInput)?;

        let test = test.parse()?;
        let operands = operands
            .split_whitespace()
            .map(|f| f.parse())
            .collect::<std::result::Result<Vec<u64>, ParseIntError>>()?;

        Ok(Equation { test, operands })
    }
}

type ResultType = u64;
type DataType = Vec<Equation>;

fn parse(input: &str) -> Result<DataType> {
    input
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Equation>>>()
}

fn task1(data: &DataType) -> Result<ResultType> {
    let operators = [Operator::Add, Operator::Multiply];

    Ok(data
        .par_iter()
        .filter(|eq| {
            operators
                .multiset(eq.operands.len() - 1)
                .any(|o| eq.test(&o))
        })
        .map(|eq| eq.test)
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];

    Ok(data
        .par_iter()
        .filter(|eq| {
            operators
                .multiset(eq.operands.len() - 1)
                .any(|o| eq.test(&o))
        })
        .map(|eq| eq.test)
        .sum())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(3749));
        assert_eq!(task2, Some(11387));
    }
}
