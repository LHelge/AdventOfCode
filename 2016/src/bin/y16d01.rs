use aoc::AoCError;
use std::str::FromStr;

#[derive(Debug)]
enum Heading {
    North,
    East,
    South,
    West,
}

impl Heading {
    fn turn(&self, m: &Move) -> Self {
        match (self, m) {
            (Heading::North, Move::Left(_)) | (Heading::South, Move::Right(_)) => Heading::West,
            (Heading::North, Move::Right(_)) | (Heading::South, Move::Left(_)) => Heading::East,
            (Heading::East, Move::Left(_)) | (Heading::West, Move::Right(_)) => Heading::North,
            (Heading::East, Move::Right(_)) | (Heading::West, Move::Left(_)) => Heading::South,
        }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Position {
    east: i32,
    north: i32,
}

impl Position {
    fn new() -> Self {
        Position { east: 0, north: 0 }
    }

    fn walk(&self, heading: &Heading, blocks: u32) -> Line {
        let end = match heading {
            Heading::North => Position {
                north: self.north + blocks as i32,
                east: self.east,
            },
            Heading::South => Position {
                north: self.north - blocks as i32,
                east: self.east,
            },
            Heading::East => Position {
                north: self.north,
                east: self.east + blocks as i32,
            },
            Heading::West => Position {
                north: self.north,
                east: self.east - blocks as i32,
            },
        };

        Line {
            start: self.clone(),
            end,
        }
    }

    fn blocks_from_origin(&self) -> u32 {
        self.east.unsigned_abs() + self.north.unsigned_abs()
    }
}

struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<Position> {
        // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection

        let (x1, y1) = (self.start.east, self.start.north);
        let (x2, y2) = (self.end.east, self.end.north);
        let (x3, y3) = (other.start.east, other.start.north);
        let (x4, y4) = (other.end.east, other.end.north);

        let d = ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)) as f64;

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64 / d;
        let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) as f64 / d;

        // Do not include starting point to prevent matching end of previous line
        if t > 0.0 && t <= 1.0 && u > 0.0 && u <= 1.0 {
            let x = x1 as f64 + t * (x2 - x1) as f64;
            let y = y1 as f64 + t * (y2 - y1) as f64;

            Some(Position {
                east: x.round() as i32,
                north: y.round() as i32,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Move {
    Left(u32),
    Right(u32),
}

impl FromStr for Move {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_at(1);
        let steps = steps.parse()?;

        match direction {
            "L" => Ok(Move::Left(steps)),
            "R" => Ok(Move::Right(steps)),
            _ => Err(AoCError::BadInput),
        }
    }
}

fn solve_task(input: &str) -> (u32, u32) {
    let moves: Vec<Move> = input
        .split(", ")
        .map(|m| m.trim().parse().expect("Bad input"))
        .collect();

    let mut heading = Heading::North;
    let mut position = Position::new();
    let mut task2: Option<u32> = None;
    let mut lines: Vec<Line> = Vec::new();

    for m in moves {
        heading = heading.turn(&m);
        let blocks = match m {
            Move::Left(b) | Move::Right(b) => b,
        };

        let line = position.walk(&heading, blocks);
        position = line.end.clone();

        if task2.is_none() {
            if let Some(intersection) = lines.iter().find_map(|l| l.intersection(&line)) {
                task2 = Some(intersection.blocks_from_origin());
            }
            lines.push(line);
        }
    }

    let task1 = position.blocks_from_origin();

    (task1, task2.unwrap_or(0))
}

fn main() {
    let input = aoc::get_input(
        2016,
        1,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d01 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task("R2, L3");
        assert_eq!(example1, 5);

        let (example1, _) = solve_task("R2, R2, R2");
        assert_eq!(example1, 2);

        let (example1, _) = solve_task("R5, L5, R5, R3");
        assert_eq!(example1, 12);

        let (_, example2) = solve_task("R8, R4, R4, R4");
        assert_eq!(example2, 4);
    }
}
