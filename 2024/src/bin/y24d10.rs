const YEAR: u16 = 2024;
const DAY: u8 = 10;
use std::collections::{HashMap, HashSet};

use aoc::*;
use vec2d::{Direction, Position, Vec2d};

type Trail = Vec<Position>;
type ResultType = usize;
type DataType = HashMap<Position, Vec<Trail>>;

fn find_trails(map: &Vec2d<u8>, trail: &mut Trail) -> Vec<Trail> {
    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut trails = vec![];

    if let Some(&pos) = trail.last() {
        if let Some(&tile) = map.get(pos) {
            for &dir in DIRECTIONS.iter() {
                let next = pos + dir.into();

                if let Some(&next_tile) = map.get(next) {
                    if next_tile == tile + 1 {
                        let mut trail = trail.clone();
                        trail.push(next);
                        if next_tile == 9 {
                            trails.push(trail);
                        } else {
                            trails.extend(find_trails(map, &mut trail));
                        }
                    }
                }
            }
        }
    }

    trails
}

fn parse(input: &str) -> Result<DataType> {
    let map = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();
    let map = Vec2d::new(map)?;

    let trails = map
        .iter()
        .filter_map(|(pos, &tile)| {
            if tile == 0 {
                let mut trail = vec![pos];
                Some((pos, find_trails(&map, &mut trail)))
            } else {
                None
            }
        })
        .collect();

    Ok(trails)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .map(|(_, trails)| {
            trails
                .iter()
                .map(|trail| *trail.last().unwrap())
                .collect::<HashSet<Position>>()
                .len()
        })
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|(_, trails)| trails.len()).sum())
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
        let input = r#"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(36));
        assert_eq!(task2, Some(81));
    }
}
