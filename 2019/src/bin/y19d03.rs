const YEAR: u16 = 2019;
const DAY: u8 = 3;

use aoc::*;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Position(i64, i64);

impl Position {
    fn up(&self) -> Position {
        Position(self.0, self.1 + 1)
    }

    fn down(&self) -> Position {
        Position(self.0, self.1 - 1)
    }

    fn right(&self) -> Position {
        Position(self.0 + 1, self.1)
    }

    fn left(&self) -> Position {
        Position(self.0 - 1, self.1)
    }

    fn manhattan(&self) -> u64 {
        self.0.unsigned_abs() + self.1.unsigned_abs()
    }

    fn is_origin(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

#[derive(Debug)]
enum Segment {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl FromStr for Segment {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let (dir, dist) = s.split_at(1);
        let dist = dist.parse().map_err(|_| AoCError::BadInput)?;

        match dir {
            "U" => Ok(Segment::Up(dist)),
            "D" => Ok(Segment::Down(dist)),
            "L" => Ok(Segment::Left(dist)),
            "R" => Ok(Segment::Right(dist)),
            _ => Err(AoCError::BadInput),
        }
    }
}

#[derive(Debug)]
struct Wire(Vec<Segment>);

impl FromStr for Wire {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Wire(
            s.split(',')
                .map(|seg| seg.parse::<Segment>())
                .collect::<std::result::Result<Vec<Segment>, AoCError>>()?,
        ))
    }
}

impl Wire {
    fn positions(&self) -> PositionIterator<'_> {
        PositionIterator {
            wire: self,
            next_position: Some(Position(0, 0)),
            segment: 0,
            segment_step: 0,
        }
    }

    fn positions_distance(&self) -> PositionDistanceIterator<'_> {
        PositionDistanceIterator {
            position: self.positions(),
            distance: 0,
        }
    }
}

struct PositionIterator<'a> {
    wire: &'a Wire,
    next_position: Option<Position>,
    segment: usize,
    segment_step: usize,
}

impl<'a> Iterator for PositionIterator<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let current_position = self.next_position?;

        let (next, length) = match self.wire.0.get(self.segment) {
            Some(Segment::Up(length)) => (Some(current_position.up()), *length),
            Some(Segment::Down(length)) => (Some(current_position.down()), *length),
            Some(Segment::Left(length)) => (Some(current_position.left()), *length),
            Some(Segment::Right(length)) => (Some(current_position.right()), *length),
            None => (None, 0usize),
        };

        self.segment_step += 1;
        if self.segment_step >= length {
            self.segment += 1;
            self.segment_step = 0;
        }
        self.next_position = next;

        Some(current_position)
    }
}

struct PositionDistanceIterator<'a> {
    position: PositionIterator<'a>,
    distance: u64,
}

impl<'a> Iterator for PositionDistanceIterator<'a> {
    type Item = (Position, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let current_distance = self.distance;
        self.distance += 1;
        Some((self.position.next()?, current_distance))
    }
}

type ResultType = u64;
type DataType = (Wire, Wire);

fn parse(input: &str) -> Result<DataType> {
    let mut lines = input.lines();
    let wire1 = lines.next().ok_or(AoCError::BadInput)?.parse()?;
    let wire2 = lines.next().ok_or(AoCError::BadInput)?.parse()?;
    Ok((wire1, wire2))
}

fn task1(data: &DataType) -> Result<ResultType> {
    let (wire1, wire2) = data;

    let wire1_positions: HashSet<Position> = wire1.positions().collect();

    wire2
        .positions()
        .filter_map(|pos| {
            if !pos.is_origin() && wire1_positions.contains(&pos) {
                Some(pos.manhattan())
            } else {
                None
            }
        })
        .min()
        .ok_or(AoCError::BadInput)
}

fn task2(data: &DataType) -> Result<ResultType> {
    let (wire1, wire2) = data;

    let wire1_positions: HashMap<Position, u64> = wire1.positions_distance().collect();

    wire2
        .positions_distance()
        .filter_map(|(pos2, dist2)| {
            if pos2.is_origin() {
                return None;
            }

            wire1_positions.get(&pos2).map(|dist1| dist1 + dist2)
        })
        .min()
        .ok_or(AoCError::BadInput)
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
    fn example1() {
        let input = r#"R8,U5,L5,D3
U7,R6,D4,L4"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(6));
        assert_eq!(task2, Some(30));
    }

    #[test]
    fn example2() {
        let input = r#"R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(159));
        assert_eq!(task2, Some(610));
    }

    #[test]
    fn example3() {
        let input = r#"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(135));
        assert_eq!(task2, Some(410));
    }
}
