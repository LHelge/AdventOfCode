use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    str::FromStr,
};

use aoc::{problem::*, utils::*, *};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Position {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.split(',');

        let x = s.next().ok_or(AoCError::BadInput)?.parse()?;
        let y = s.next().ok_or(AoCError::BadInput)?.parse()?;
        let z = s.next().ok_or(AoCError::BadInput)?.parse()?;

        Ok(Position::new(x, y, z))
    }
}

impl Position {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Position { x, y, z }
    }

    fn distance_to(&self, other: &Position) -> f64 {
        let x = self.x.abs_diff(other.x) as f64;
        let y = self.y.abs_diff(other.y) as f64;
        let z = self.z.abs_diff(other.z) as f64;

        (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct JunctionBox {
    pos: Position,
    circuit: Option<usize>,
}

impl FromStr for JunctionBox {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(JunctionBox::new(s.parse()?))
    }
}

impl Hash for JunctionBox {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialEq for JunctionBox {
    fn eq(&self, other: &Self) -> bool {
        self.pos.eq(&other.pos)
    }
}

impl JunctionBox {
    fn new(pos: Position) -> Self {
        Self { pos, circuit: None }
    }
}

struct JuctionBoxSet(HashSet<JunctionBox>);

impl JuctionBoxSet {
    fn connect(&mut self, box1: JunctionBox, box2: JuctionBox) {
        
    }
}

#[derive(Default)]
struct Problem {
    boxes: HashSet<JunctionBox>,
}

impl Problem {
    fn connect(&mut self)
}

impl AoCProblem<u64, u64> for Problem {
    fn date() -> Date {
        Date::new(2025, 8).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        let positions: Vec<Position> = input.parse_lines()?;
        self.boxes = positions.iter().map(|pos| JunctionBox::new(*pos)).collect();
        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut boxes = self.boxes.clone();

        for _ in 0..10 {
            let closest

        }
    }
}

fn main() -> Result<()> {
    let mut problem = Problem::default();
    let solution = problem.solve()?;

    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(40);
    }
}
