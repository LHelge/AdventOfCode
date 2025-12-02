const YEAR: u16 = 2025;
const DAY: u8 = 2;
use std::{ops::RangeInclusive, str::FromStr};

use aoc::{parser::Parser, *};

type ResultType = u64;
type DataType = Vec<IdRange>;

trait IdValidator {
    fn is_valid1(&self) -> bool;
    fn is_valid2(&self) -> bool;
}

impl IdValidator for u64 {
    fn is_valid1(&self) -> bool {
        let s = self.to_string();
        let (first, last) = s.split_at(s.len() / 2);

        if first == last {
            //println!("invalid: {s} ({first} {last})");
            return false;
        }
        true
    }

    fn is_valid2(&self) -> bool {
        let s = self.to_string();
        let s = s.as_bytes();

        for l in 1..=(s.len() / 2) {
            if !s.len().is_multiple_of(l) {
                continue;
            }

            let seq = s.chunks(l).next().unwrap();
            if s.chunks(l).all(|c| c == seq) {
                //println!("invalid: {self}");
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
struct IdRange(RangeInclusive<u64>);

impl FromStr for IdRange {
    type Err = AoCError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(AoCError::BadInput)?;
        let start: u64 = start.parse().map_err(|_| AoCError::BadInput)?;
        let end: u64 = end.parse().map_err(|_| AoCError::BadInput)?;
        Ok(IdRange(start..=end))
    }
}

impl IdRange {
    fn sum_invalid1(&self) -> u64 {
        let range = self.clone();

        range.0.filter(|id| !id.is_valid1()).sum()
    }

    fn sum_invalid2(&self) -> u64 {
        let range = self.clone();

        range.0.filter(|id| !id.is_valid2()).sum()
    }
}

fn parse(input: &str) -> Result<DataType> {
    input.parse_delimited(',')
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|range| range.sum_invalid1()).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|range| range.sum_invalid2()).sum())
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
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1227775554));
        assert_eq!(task2, Some(4174379265));
    }
}
