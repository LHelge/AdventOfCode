const YEAR: u16 = 2025;
const DAY: u8 = 1;

use aoc::parser::Parser;
use aoc::*;
use std::ops::RangeInclusive;
use std::str::FromStr;

struct Dial {
    position: i64,
    range: RangeInclusive<i64>,
}

impl Dial {
    fn new(start: i64, range: RangeInclusive<i64>) -> Self {
        Self {
            position: start,
            range,
        }
    }

    fn turn(&mut self, rotation: &Rotation) -> usize {
        let (steps, dir, reset) = match rotation {
            Rotation::Left(steps) => (*steps, -1i64, *self.range.end()),
            Rotation::Right(steps) => (*steps, 1i64, *self.range.start()),
        };

        let mut zeros = 0;

        for _ in 0..steps {
            self.position += dir;

            if !self.range.contains(&self.position) {
                self.position = reset;
            }

            if self.is_zero() {
                zeros += 1;
            }
        }

        zeros
    }

    fn is_zero(&self) -> bool {
        self.position == 0
    }
}

#[derive(Debug)]
enum Rotation {
    Left(usize),
    Right(usize),
}

impl FromStr for Rotation {
    type Err = AoCError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.chars().next().ok_or(AoCError::BadInput)? {
            'L' => Ok(Self::Left(s[1..].parse().map_err(|_| AoCError::BadInput)?)),
            'R' => Ok(Self::Right(s[1..].parse().map_err(|_| AoCError::BadInput)?)),
            _ => Err(AoCError::BadInput),
        }
    }
}

type ResultType = usize;
type DataType = Vec<Rotation>;

fn parse(input: &str) -> Result<DataType> {
    input.parse_lines()
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut dial = Dial::new(50, 0..=99);
    let mut password = 0;
    for rotation in data {
        _ = dial.turn(rotation);
        if dial.is_zero() {
            password += 1;
        }
    }

    Ok(password)
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut dial = Dial::new(50, 0..=99);
    let mut password = 0;

    for rotation in data {
        password += dial.turn(rotation);
    }
    Ok(password)
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
        let input = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(3));
        assert_eq!(task2, Some(6));
    }
}
