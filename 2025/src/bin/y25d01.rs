const YEAR: u16 = 2025;
const DAY: u8 = 1;
use std::str::FromStr;

use aoc::parser::Parser;
use aoc::*;

#[derive(Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
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
    let mut dial = 50;
    let mut password = 0usize;

    for rotation in data {
        dial += match rotation {
            Rotation::Left(steps) => -*steps,
            Rotation::Right(steps) => *steps,
        };

        while dial < 0 {
            dial += 100
        }
        while dial > 99 {
            dial -= 100
        }

        if dial == 0 {
            password += 1;
        }
    }
    Ok(password)
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut dial = 50;
    let mut password = 0usize;

    for rotation in data {
        dial += match rotation {
            Rotation::Left(0) => continue,
            Rotation::Right(0) => continue,
            Rotation::Left(steps) => -*steps,
            Rotation::Right(steps) => *steps,
        };

        print!("dial: {dial}");

        if dial == 0 {
            password += 1;
        }

        while dial < 0 {
            dial += 100;
            password += 1;
            print!("spin-");
        }

        while dial > 99 {
            dial -= 100;
            password += 1;
            print!("spin+");
        }

        println!("{rotation:?}: {dial}");
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
