const YEAR: u16 = 2025;
const DAY: u8 = 5;

use aoc::{parser::Parser, *};
use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug, Clone)]
struct IdRange(RangeInclusive<u64>);

impl FromStr for IdRange {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s.split_once('-').ok_or(AoCError::BadInput)?;

        Ok(IdRange(RangeInclusive::new(start.parse()?, end.parse()?)))
    }
}

impl IdRange {
    fn merge(self, other: IdRange) -> Result<IdRange> {
        if !self.overlaps(&other) {
            Err(AoCError::BadInput)?;
        }

        let start = *self.0.start().min(other.0.start());
        let end = *self.0.end().max(other.0.end());

        Ok(IdRange(start..=end))
    }

    fn overlaps(&self, other: &IdRange) -> bool {
        self.0.contains(other.0.start())
            || self.0.contains(other.0.end())
            || other.0.contains(self.0.start())
            || other.0.contains(self.0.end())
    }

    fn contains(&self, id: &u64) -> bool {
        self.0.contains(id)
    }

    fn len(&self) -> usize {
        self.0.start().abs_diff(*self.0.end()) as usize + 1
    }
}

#[derive(Debug)]
struct IdDatabase(Vec<IdRange>);

impl IdDatabase {
    fn new() -> Self {
        IdDatabase(Vec::new())
    }

    fn insert(&mut self, range: IdRange) -> Result<()> {
        if let Some(index) = self.0.iter().position(|r| range.overlaps(r)) {
            let found = self.0.swap_remove(index);
            self.insert(found.merge(range)?)
        } else {
            self.0.push(range);
            Ok(())
        }
    }

    fn len(&self) -> usize {
        self.0.iter().map(|r| r.len()).sum()
    }

    fn contains(&self, id: &u64) -> bool {
        self.0.iter().any(|r| r.contains(id))
    }
}

type ResultType = usize;
type DataType = (IdDatabase, Vec<u64>);

fn parse(input: &str) -> Result<DataType> {
    let (ranges, ids) = input.split_once("\n\n").ok_or(AoCError::BadInput)?;

    let mut db = IdDatabase::new();
    for range in ranges.parse_lines()? {
        db.insert(range)?;
    }

    let ids = ids.parse_lines()?;
    Ok((db, ids))
}

fn task1(data: &DataType) -> Result<ResultType> {
    let (db, ids) = data;
    Ok(ids.iter().filter(|id| db.contains(id)).count())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let (db, _) = data;
    Ok(db.len())
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
        let input = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(3));
        assert_eq!(task2, Some(14));
    }
}
