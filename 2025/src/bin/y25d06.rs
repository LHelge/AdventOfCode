use aoc::{problem::*, utils::*, *};
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
struct MathProblem {
    numbers: Vec<u64>,
    op: Operation,
}

impl MathProblem {
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

struct Problem {
    lines: Vec<String>,
    problems: Vec<MathProblem>,
}

impl AoCProblem for Problem {
    type Solution1 = u64;
    type Solution2 = u64;

    const YEAR: u16 = 2025;
    const DAY: u8 = 6;

    fn parse(input: &str) -> Result<Self> {
        let lines: Vec<&str> = input.lines().collect();

        let operations: Vec<_> = lines
            .last()
            .ok_or(AoCError::BadInput)?
            .parse_whitespace_delimited()?;

        Ok(Self {
            lines: lines[..(lines.len() - 1)]
                .iter()
                .map(|&s| s.to_owned())
                .collect(),
            problems: operations.iter().map(|op| MathProblem::new(*op)).collect(),
        })
    }

    fn part1(&self) -> Result<u64> {
        let mut problems = self.problems.clone();

        for numbers in self.lines.iter() {
            for (i, num) in numbers.split_whitespace().enumerate() {
                problems[i].push_number(num)?;
            }
        }

        Ok(problems.iter().map(|problem| problem.calculate()).sum())
    }

    fn part2(&self) -> Result<u64> {
        let mut problems = self.problems.clone();

        let string_length = self.lines.first().ok_or(AoCError::BadInput)?.len();
        let mut problem_idx = 0;

        for i in 0..string_length {
            let num: String = self.lines.iter().map(|s| s.as_bytes()[i] as char).collect();

            if num.trim().is_empty() {
                problem_idx += 1;
            } else {
                problems[problem_idx].push_number(num.trim())?;
            }
        }

        Ok(problems.iter().map(|problem| problem.calculate()).sum())
    }
}

fn main() -> Result<()> {
    Problem::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<()> {
        let input = r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#;

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part1()?, 4277556);
        assert_eq!(problem.part2()?, 3263827);

        Ok(())
    }
}
