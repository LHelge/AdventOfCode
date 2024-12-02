use aoc::AoCInput;
use std::ops::Add;

#[derive(Debug, Default)]
struct Position {
    right: usize,
    down: usize,
}

#[derive(Debug, Default)]
struct Step {
    right: i32,
    down: i32,
}

impl Step {
    fn new(right: i32, down: i32) -> Self {
        Self { right, down }
    }
}

impl Add<&Step> for Position {
    type Output = Position;

    fn add(self, rhs: &Step) -> Self::Output {
        Self {
            right: self.right.wrapping_add_signed(rhs.right as isize),
            down: self.down.wrapping_add_signed(rhs.down as isize),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MapSquare {
    Open,
    Tree,
}

impl From<char> for MapSquare {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Open,
            '#' => Self::Tree,
            _ => panic!("Bad input"),
        }
    }
}

struct Map {
    map: Vec<Vec<MapSquare>>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut map = vec![];

        for line in value.lines() {
            map.push(line.chars().map(|c| c.into()).collect());
        }

        Self { map }
    }
}

impl Map {
    fn get(&self, pos: &Position) -> Option<&MapSquare> {
        if let Some(row) = self.map.get(pos.down) {
            let index = pos.right % row.len();
            row.get(index)
        } else {
            None
        }
    }
}

fn evaluate_slope(map: &Map, step: &Step) -> usize {
    let mut pos = Position::default();

    let mut trees = 0;
    while let Some(square) = map.get(&pos) {
        if square == &MapSquare::Tree {
            trees += 1;
        }
        pos = pos + step;
    }

    trees
}

fn solve_task(input: &str) -> (usize, usize) {
    let map: Map = input.into();

    let task1_slope = Step::new(3, 1);
    let task1 = evaluate_slope(&map, &task1_slope);

    let task2_slopes = [
        Step::new(1, 1),
        Step::new(3, 1),
        Step::new(5, 1),
        Step::new(7, 1),
        Step::new(1, 2),
    ];
    let task2 = task2_slopes
        .iter()
        .map(|step| evaluate_slope(&map, step))
        .product();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2020, 3)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2020d03 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;
        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 7);
        assert_eq!(example2, 336);
    }
}
