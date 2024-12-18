const YEAR: u16 = 2024;
const DAY: u8 = 16;
use std::{collections::{BTreeMap, HashMap}, fmt::Display, hash::RandomState, u64};

use aoc::*;
use nom::Err;
use vec2d::{Direction, Position, Vec2d};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Start,
    End,
    Move(Direction),
}

impl TryFrom<char> for Tile {
    type Error = AoCError;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            c => Err(AoCError::BadCharacter(c)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Start => write!(f, "S"),
            Tile::End => write!(f, "E"),
            Tile::Move(d) => write!(f, "{d}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Move,
    TurnCW,
    TurnCCW,
}

impl Action {
    fn score(&self) -> u64 {
        match self {
            Self::Move => 1,
            _ => 1000,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Reindeer {
    position: Position,
    direction: Direction,
}

impl Reindeer {
    fn new(position: Position) -> Self {
        Self {
            position,
            direction: Direction::East,
        }
    }

    fn peek_forward(&self) -> Position {
        self.position + self.direction.into()
    }

    fn act(&self, action: Action) -> Reindeer {
        match action {
            Action::Move => Self { position: self.position + self.direction.into(), direction: self.direction },
            Action::TurnCW => Self { position: self.position, direction: self.direction.turn_cw().turn_cw() },
            Action::TurnCCW => Self { position: self.position, direction: self.direction.turn_ccw().turn_ccw() },
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    map: Vec2d<Tile>,
    end: Position,
    reindeer: Reindeer,
    path: Vec<Action>,
}

impl Maze {
    fn new(map: Vec2d<Tile>) -> Result<Self> {
        let start = map.find_position(&Tile::Start).ok_or(AoCError::BadInput)?;
        let end = map.find_position(&Tile::End).ok_or(AoCError::BadInput)?;
        let reindeer = Reindeer::new(start);
        let path = vec![];

        Ok(Self {
            map,
            end,
            reindeer,
            path,
        })
    }

    fn act(&mut self, action: Action) {
        self.reindeer = self.reindeer.act(action);
        self.path.push(action);
        self.map
            .set(self.reindeer.position, Tile::Move(self.reindeer.direction))
            .unwrap();
    }

    fn score(&self) -> u64 {
        self.path.iter().map(Action::score).sum()
    }
}

type ResultType = u64;
type DataType = Maze;


fn parse(input: &str) -> Result<DataType> {
    Maze::new(input.parse()?)
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut maze = data.clone();

    // Todo implement pathfinding

    maze.act(Action::TurnCCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::TurnCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::TurnCCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::TurnCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::TurnCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::TurnCCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::TurnCCW);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);
    maze.act(Action::Move);

    println!("{}", maze.map);
    Ok(maze.score())
}

fn task2(_data: &DataType) -> Result<ResultType> {
    Err(AoCError::Unsolved)
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
        let input = r#"###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(7036));
        assert_eq!(task2, None);
    }
}
