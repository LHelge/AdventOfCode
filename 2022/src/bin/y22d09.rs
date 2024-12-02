use std::{cmp::Ordering, collections::HashSet, str::FromStr};

use aoc::{AoCError, AoCInput};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    dir: Direction,
    dist: i32,
}

impl FromStr for Move {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').unwrap();
        let dist = dist.parse::<i32>()?;

        match dir {
            "U" => Ok(Move {
                dir: Direction::Up,
                dist,
            }),
            "D" => Ok(Move {
                dir: Direction::Down,
                dist,
            }),
            "L" => Ok(Move {
                dir: Direction::Left,
                dist,
            }),
            "R" => Ok(Move {
                dir: Direction::Right,
                dist,
            }),
            _ => Err(AoCError::BadInput),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Hash, Eq, Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Rope<const N: usize> {
    knots: [Position; N],
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        Rope {
            knots: [Position { x: 0, y: 0 }; N],
        }
    }

    fn move_head(&mut self, dir: &Direction) {
        // Move head in the direction
        match dir {
            Direction::Up => self.knots[0].y += 1,
            Direction::Down => self.knots[0].y -= 1,
            Direction::Right => self.knots[0].x += 1,
            Direction::Left => self.knots[0].x -= 1,
        }

        // Move all following knots according to the rules
        for i in 1..N {
            // Predecessor is above/below
            if self.knots[i - 1].y.abs_diff(self.knots[i].y) > 1 {
                // Move up/down
                self.knots[i].y += match self.knots[i - 1].y.cmp(&self.knots[i].y) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => unreachable!(),
                };

                // Move diagonally if needed
                self.knots[i].x += match self.knots[i - 1].x.cmp(&self.knots[i].x) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                };
            }

            // Predecessor is to the left/right
            if self.knots[i - 1].x.abs_diff(self.knots[i].x) > 1 {
                // Move left/right
                self.knots[i].x += match self.knots[i - 1].x.cmp(&self.knots[i].x) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => unreachable!(),
                };

                // Move diagonally if needed
                self.knots[i].y += match self.knots[i - 1].y.cmp(&self.knots[i].y) {
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                };
            }
        }
    }

    fn get_tail(&self) -> &Position {
        self.knots.last().unwrap()
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let moves: Vec<Move> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut rope1: Rope<2> = Rope::new();
    let mut rope2: Rope<10> = Rope::new();
    let mut visited1: HashSet<Position> = HashSet::new();
    let mut visited2: HashSet<Position> = HashSet::new();
    for m in moves.iter() {
        for _ in 0..m.dist {
            rope1.move_head(&m.dir);
            visited1.insert(*rope1.get_tail());

            rope2.move_head(&m.dir);
            visited2.insert(*rope2.get_tail());
        }
    }
    let task1 = visited1.len();
    let task2 = visited2.len();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2022, 9)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d09 {
    use super::*;

    #[test]
    fn examples1() {
        let example_input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let (example1, _) = solve_task(example_input);
        assert_eq!(example1, 13);
    }

    #[test]
    fn examples2() {
        let example_input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let (_, example2) = solve_task(example_input);
        assert_eq!(example2, 36);
    }
}
