use aoc::{AoCError, AoCInput};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
    Inside,
    Outside,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Pipe::NorthSouth,
            '-' => Pipe::EastWest,
            'L' => Pipe::NorthEast,
            'J' => Pipe::NorthWest,
            '7' => Pipe::SouthWest,
            'F' => Pipe::SouthEast,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => panic!("Invalid pipe: {}", value),
        }
    }
}

impl From<&Pipe> for char {
    fn from(value: &Pipe) -> char {
        match value {
            Pipe::NorthSouth => '│',
            Pipe::EastWest => '─',
            Pipe::NorthEast => '└',
            Pipe::NorthWest => '┘',
            Pipe::SouthWest => '┐',
            Pipe::SouthEast => '┌',
            Pipe::Ground => '.',
            Pipe::Start => 'S',
            Pipe::Inside => 'I',
            Pipe::Outside => 'O',
        }
    }
}

impl Pipe {
    fn ends(&self, position: Position) -> Option<(Option<Position>, Option<Position>)> {
        match self {
            Pipe::NorthSouth => Some((position.north(), position.south())),
            Pipe::EastWest => Some((position.east(), position.west())),
            Pipe::NorthEast => Some((position.north(), position.east())),
            Pipe::NorthWest => Some((position.north(), position.west())),
            Pipe::SouthWest => Some((position.south(), position.west())),
            Pipe::SouthEast => Some((position.south(), position.east())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    east: usize,
    south: usize,
}

impl Position {
    fn new(east: usize, south: usize) -> Self {
        Position { east, south }
    }

    fn north(&self) -> Option<Position> {
        if self.south == 0 {
            None
        } else {
            Some(Self::new(self.east, self.south - 1))
        }
    }

    fn south(&self) -> Option<Position> {
        Some(Self::new(self.east, self.south + 1))
    }

    fn west(&self) -> Option<Position> {
        if self.east == 0 {
            None
        } else {
            Some(Self::new(self.east - 1, self.south))
        }
    }

    fn east(&self) -> Option<Position> {
        Some(Self::new(self.east + 1, self.south))
    }
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<Pipe>>,
    start: Position,
}

impl FromStr for Maze {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = Vec::new();
        let mut start = Position::new(0, 0);

        for (south, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (east, c) in line.chars().enumerate() {
                let pipe = Pipe::from(c);
                if pipe == Pipe::Start {
                    start = Position::new(east, south);
                }
                row.push(pipe);
            }
            map.push(row);
        }

        let mut maze = Maze { map, start };

        maze.replace_start();

        Ok(maze)
    }
}

impl Maze {
    /// Get pipe at a position
    fn get(&self, position: Position) -> Option<Pipe> {
        self.map
            .get(position.south)
            .and_then(|east| east.get(position.east))
            .copied()
    }

    /// Replace start with correct pipe for the loop to form
    fn replace_start(&mut self) {
        let north = self.start.north().and_then(|p| self.get(p));
        let south = self.start.south().and_then(|p| self.get(p));
        let west = self.start.west().and_then(|p| self.get(p));
        let east = self.start.east().and_then(|p| self.get(p));

        let north = north == Some(Pipe::NorthSouth)
            || north == Some(Pipe::SouthEast)
            || north == Some(Pipe::SouthWest);
        let south = south == Some(Pipe::NorthSouth)
            || south == Some(Pipe::NorthEast)
            || south == Some(Pipe::NorthWest);
        let west = west == Some(Pipe::EastWest)
            || west == Some(Pipe::NorthEast)
            || west == Some(Pipe::SouthEast);
        let east = east == Some(Pipe::EastWest)
            || east == Some(Pipe::NorthWest)
            || east == Some(Pipe::SouthWest);

        self.map[self.start.south][self.start.east] = match (north, south, east, west) {
            (true, true, false, false) => Pipe::NorthSouth,
            (false, false, true, true) => Pipe::EastWest,
            (true, false, true, false) => Pipe::NorthEast,
            (true, false, false, true) => Pipe::NorthWest,
            (false, true, false, true) => Pipe::SouthWest,
            (false, true, true, false) => Pipe::SouthEast,
            _ => panic!("Invalid start position"),
        };
    }

    /// Create an iterator that will walk the loop from the start
    fn iter(&self) -> MazeWalker {
        MazeWalker {
            maze: self,
            position: self.start,
            prev_position: None,
        }
    }
}

struct MazeWalker<'a> {
    maze: &'a Maze,
    position: Position,
    prev_position: Option<Position>,
}

impl<'a> Iterator for MazeWalker<'a> {
    type Item = (Position, Pipe);

    fn next(&mut self) -> Option<Self::Item> {
        let pipe = self.maze.get(self.position)?;

        match (pipe, self.position, self.prev_position) {
            (Pipe::Ground, _, _) => None, // ended up outside of maze
            (pipe, pos, None) => {
                // First iteration
                let ends = pipe.ends(pos).unwrap();

                self.prev_position = ends.0;
                let pipe = self.maze.get(self.position).unwrap();
                Some((self.position, pipe))
            }
            (Pipe::Start, _, Some(_)) => None, // Reached start again
            (pipe, pos, Some(prev)) => {
                // continue walkking maze

                let ends = pipe.ends(pos).unwrap();

                match ends {
                    (Some(e1), Some(e2)) if e1 == prev => {
                        // continue in same direction
                        self.position = e2;
                    }
                    (Some(e1), Some(e2)) if e2 == prev => {
                        // continue in same direction
                        self.position = e1;
                    }
                    _ => panic!("Invalid maze"), // ended up in a corner
                }
                self.prev_position = Some(pos);

                let pipe = self.maze.get(self.position).unwrap();

                if self.position == self.maze.start {
                    None
                } else {
                    Some((self.position, pipe))
                }
            }
        }
    }
}

enum PipeState {
    NotOn,
    OnFromNorth,
    OnFromSouth,
}

fn anotate_inside_outside(maze: &mut [Vec<Pipe>]) {
    for line in maze.iter_mut() {
        let mut pipe_state = PipeState::NotOn;
        let mut inside = false;
        for p in line.iter_mut() {
            *p = match (&p, &pipe_state, inside) {
                (Pipe::Ground, _, false) => Pipe::Outside,
                (Pipe::Ground, _, true) => Pipe::Inside,
                (Pipe::NorthSouth, _, io) => {
                    inside = !io;
                    Pipe::NorthSouth
                }
                (Pipe::NorthEast, PipeState::NotOn, _) => {
                    pipe_state = PipeState::OnFromNorth;
                    Pipe::NorthEast
                }
                (Pipe::SouthEast, PipeState::NotOn, _) => {
                    pipe_state = PipeState::OnFromSouth;
                    Pipe::SouthEast
                }
                (Pipe::NorthWest, PipeState::OnFromSouth, io) => {
                    inside = !io;
                    pipe_state = PipeState::NotOn;
                    Pipe::NorthWest
                }
                (Pipe::NorthWest, _, _) => {
                    pipe_state = PipeState::NotOn;
                    Pipe::NorthWest
                }
                (Pipe::SouthWest, PipeState::OnFromNorth, io) => {
                    inside = !io;
                    pipe_state = PipeState::NotOn;
                    Pipe::SouthWest
                }
                (Pipe::SouthWest, _, _) => {
                    pipe_state = PipeState::NotOn;
                    Pipe::SouthWest
                }
                _ => *p,
            }
        }
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let maze: Maze = input.parse().unwrap();

    let maze_loop = maze.iter().collect::<Vec<_>>();

    // Create a new map with only the loop set
    let mut new_map: Vec<Vec<Pipe>> = vec![vec![Pipe::Ground; maze.map[0].len()]; maze.map.len()];
    for (pos, pipe) in maze_loop.iter() {
        new_map[pos.south][pos.east] = *pipe;
    }

    // Anotate inside and outside
    anotate_inside_outside(&mut new_map);

    // Print maze
    // for line in new_map.iter() {
    //     println!(
    //         "{}",
    //         line.iter()
    //             .map(<&Pipe as Into<char>>::into)
    //             .collect::<String>()
    //     );
    // }

    let task1 = maze_loop.len() / 2;
    let task2 = new_map
        .iter()
        .flatten()
        .filter(|p| **p == Pipe::Inside)
        .count();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2023, 10)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d10 {
    use super::*;

    #[test]
    fn examples() {
        let example_input1 = r#"-L|F7
7S-7|
L|7||
-L-J|
L|-JF"#;

        let (example1, example2) = solve_task(example_input1);
        assert_eq!(example1, 4);
        assert_eq!(example2, 1);

        let example_input2 = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

        let (example1, example2) = solve_task(example_input2);
        assert_eq!(example1, 8);
        assert_eq!(example2, 1);
    }

    #[test]
    fn examples2() {
        let example_input1 = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

        let (example1, example2) = solve_task(example_input1);
        assert_eq!(example1, 23);
        assert_eq!(example2, 4);

        let example_input1 = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

        let (example1, example2) = solve_task(example_input1);
        assert_eq!(example1, 70);
        assert_eq!(example2, 8);

        let example_input1 = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;

        let (example1, example2) = solve_task(example_input1);
        assert_eq!(example1, 80);
        assert_eq!(example2, 10);
    }
}
