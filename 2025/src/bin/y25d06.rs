const YEAR: u16 = 2025;
const DAY: u8 = 6;

use aoc::{parser::Parser, *};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl FromStr for Operation {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Mul),
            _ => Err(AoCError::BadInput),
        }
    }
}

#[derive(Debug, Clone)]
struct Problem {
    numbers: Vec<u64>,
    op: Operation,
}

impl Problem {
    fn new(op: Operation) -> Self {
        Self {
            numbers: vec![],
            op,
        }
    }

    fn push_number(&mut self, num: &str) -> Result<()> {
        self.numbers.push(num.parse()?);
        Ok(())
    }

    fn calculate(&self) -> u64 {
        match self.op {
            Operation::Add => self.numbers.iter().sum(),
            Operation::Mul => self.numbers.iter().product(),
        }
    }
}

type ResultType = u64;
type DataType = (Vec<String>, Vec<Problem>);

fn parse(input: &str) -> Result<DataType> {
    let lines: Vec<&str> = input.lines().collect();

    let numbers = lines[..(lines.len() - 1)]
        .iter()
        .map(|&s| s.to_owned())
        .collect();

    let operations = lines
        .last()
        .ok_or(AoCError::BadInput)?
        .parse_whitespace_delilited()?;

    let problems: Vec<Problem> = operations.iter().map(|op| Problem::new(*op)).collect();

    Ok((numbers, problems))
}

fn task1(data: &DataType) -> Result<ResultType> {
    let (numbers, mut problems) = data.clone();

    for numbers in numbers {
        for (i, num) in numbers.split_whitespace().enumerate() {
            problems[i].push_number(num)?;
        }
    }

    Ok(problems.iter().map(|problem| problem.calculate()).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let (numbers, mut problems) = data.clone();

    let string_length = numbers.first().ok_or(AoCError::BadInput)?.len();
    let mut problem_idx = 0;

    for i in 0..string_length {
        let num: String = numbers.iter().map(|s| s.as_bytes()[i] as char).collect();

        if num.trim().is_empty() {
            problem_idx += 1;
        } else {
            problems[problem_idx].push_number(num.trim())?;
        }
    }

    Ok(problems.iter().map(|problem| problem.calculate()).sum())
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
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(4277556));
        assert_eq!(task2, Some(3263827));
    }
}
