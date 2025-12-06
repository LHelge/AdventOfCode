const YEAR: u16 = 2024;
const DAY: u8 = 6;
use aoc::utils::*;
use aoc::*;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
enum MapPoint {
    Empty,
    Guard,
    Visited,
    Obstruction,
}

impl TryFrom<char> for MapPoint {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '.' => Ok(MapPoint::Empty),
            '^' => Ok(MapPoint::Guard),
            '#' => Ok(MapPoint::Obstruction),
            _ => Err(AoCError::BadInput),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    position: Position,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec2d<MapPoint>,
    guard: Guard,
}

impl Map {
    fn move_guard(&mut self) -> Result<()> {
        let mut new_position = self.guard.position + self.guard.direction.into();

        while matches!(self.map.get(new_position), Some(MapPoint::Obstruction)) {
            self.guard.direction = self.guard.direction.turn_cv().turn_cv();
            new_position = self.guard.position + self.guard.direction.into();
        }

        self.guard.position = new_position;
        self.map.set(new_position, MapPoint::Visited)?;

        Ok(())
    }
}

type ResultType = usize;
type DataType = Map;

fn parse(input: &str) -> Result<DataType> {
    let data = input
        .lines()
        .map(|l| l.chars().map(|c| c.try_into()).collect())
        .collect::<Result<Vec<Vec<MapPoint>>>>()?;
    let mut map = Vec2d::new(data)?;

    if let Some((position, _)) = map.find(&MapPoint::Guard) {
        let guard = Guard {
            position,
            direction: Direction::North,
        };

        map.set(position, MapPoint::Visited)?;

        Ok(Map { map, guard })
    } else {
        Err(AoCError::BadInput)
    }
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut data = data.clone();

    while data.move_guard().is_ok() {}

    Ok(data
        .map
        .flatten()
        .filter(|&p| *p == MapPoint::Visited)
        .count())
}

fn task2(data: &DataType) -> Result<ResultType> {
    // Brute force through all possible new obstructions
    let task2 = data
        .map
        .size()
        .iter()
        .collect::<Vec<Position>>()
        .par_iter()
        .filter(|&pos| {
            // Skip if there's already an obstruction
            if matches!(data.map.get(*pos), Some(MapPoint::Obstruction)) {
                return false;
            }

            let mut data = data.clone();
            data.map.set(*pos, MapPoint::Obstruction).ok();

            let mut visited: HashSet<Guard> = HashSet::new();

            while data.move_guard().is_ok() {
                if !visited.insert(data.guard) {
                    return true;
                }
            }

            false
        })
        .count();

    Ok(task2)
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
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(41));
        assert_eq!(task2, Some(6));
    }
}
