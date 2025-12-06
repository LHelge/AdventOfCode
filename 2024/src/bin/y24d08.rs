const YEAR: u16 = 2024;
const DAY: u8 = 8;
use aoc::utils::*;
use aoc::*;
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
enum MapPoint {
    #[default]
    Empty,
    Antenna(char),
}

impl Display for MapPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapPoint::Empty => write!(f, "."),
            MapPoint::Antenna(c) => write!(f, "{}", c),
        }
    }
}

impl From<char> for MapPoint {
    fn from(value: char) -> Self {
        match value {
            '.' => MapPoint::Empty,
            c => MapPoint::Antenna(c),
        }
    }
}

struct Map {
    map: Vec2d<MapPoint>,
    unique: HashSet<MapPoint>,
}

type ResultType = usize;
type DataType = Map;

fn parse(input: &str) -> Result<DataType> {
    let data = input
        .lines()
        .map(|line| line.chars().map(MapPoint::from).collect())
        .collect();
    let map = Vec2d::new(data)?;

    let unique: HashSet<MapPoint> = map
        .iter()
        .filter_map(|(_, point)| match point {
            MapPoint::Antenna(_) => Some(*point),
            _ => None,
        })
        .collect();

    Ok(Map { map, unique })
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for frequency in data.unique.iter() {
        for comb in data
            .map
            .iter()
            .filter(|(_, f)| f == &frequency)
            .combinations(2)
        {
            let (a, b) = (comb[0].0, comb[1].0);
            let distance = a - b;

            let anti_a = a + distance;
            let anti_b = b - distance;

            if data.map.size().contains(anti_a) {
                antinodes.insert(anti_a);
            }
            if data.map.size().contains(anti_b) {
                antinodes.insert(anti_b);
            }
        }
    }

    Ok(antinodes.len())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut antinodes: HashSet<Position> = HashSet::new();

    for frequency in data.unique.iter() {
        for comb in data
            .map
            .iter()
            .filter(|(_, f)| f == &frequency)
            .combinations(2)
        {
            let (a, b) = (comb[0].0, comb[1].0);
            let distance = a - b;

            // Antennas are included
            let mut anti_a = a;
            while data.map.size().contains(anti_a) {
                antinodes.insert(anti_a);
                anti_a = anti_a + distance;
            }

            let mut anti_b = b;
            while data.map.size().contains(anti_b) {
                antinodes.insert(anti_b);
                anti_b = anti_b - distance;
            }
        }
    }

    Ok(antinodes.len())
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
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(14));
        assert_eq!(task2, Some(34));
    }
}
