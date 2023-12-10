use std::str::FromStr;

use aoc::AoCError;

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

        Ok(Maze { map, start })
    }
}

impl Maze {
    fn get(&self, position: Position) -> Option<Pipe> {
        self.map
            .get(position.south)
            .and_then(|east| east.get(position.east))
            .copied()
    }

    fn start_ends(&self) -> Vec<Position> {
        let mut positions = vec![];

        // Assume start is not on the north or west edge
        let north = self.start.north().and_then(|n| self.get(n));
        let south = self.start.south().and_then(|n| self.get(n));
        let west = self.start.west().and_then(|n| self.get(n));
        let east = self.start.east().and_then(|n| self.get(n));

        if north == Some(Pipe::NorthSouth)
            || north == Some(Pipe::SouthEast)
            || north == Some(Pipe::SouthWest)
        {
            positions.push(self.start.north().unwrap());
        }
        if south == Some(Pipe::NorthSouth)
            || south == Some(Pipe::NorthEast)
            || south == Some(Pipe::NorthWest)
        {
            positions.push(self.start.south().unwrap());
        }
        if west == Some(Pipe::EastWest)
            || west == Some(Pipe::NorthEast)
            || west == Some(Pipe::SouthEast)
        {
            positions.push(self.start.west().unwrap());
        }
        if east == Some(Pipe::EastWest)
            || east == Some(Pipe::NorthWest)
            || east == Some(Pipe::SouthWest)
        {
            positions.push(self.start.east().unwrap());
        }

        positions
    }

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
            (Pipe::Start, pos, None) => {
                // First iteration
                let ends = self.maze.start_ends();
                if ends.len() != 2 {
                    panic!("Invalid start position");
                }

                // Stays at the start and assume one of the ends as the previous position
                self.position = ends[0];
                self.prev_position = Some(pos);
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

                if pipe == Pipe::Start {
                    None
                } else {
                    Some((self.position, pipe))
                }
            }
            _ => panic!("Invalid maze"),
        }
    }
}

enum PipeState {
    NotOn,
    OnFromNorth,
    OnFromSouth,
}

fn anotate_inside_outside(maze: &mut Vec<Vec<Pipe>>) {
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
    // Set the start position
    new_map[maze.start.south][maze.start.east] = Pipe::Start;

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

    let task1 = maze_loop.len() / 2 + 1;
    let task2 = new_map
        .iter()
        .flatten()
        .filter(|p| **p == Pipe::Inside)
        .count();

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2023,
        10,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

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

        let (example1, _example2) = solve_task(example_input1);
        assert_eq!(example1, 4);
        //assert_eq!(example2, 0);

        let example_input2 = r#"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"#;

        let (example1, _example2) = solve_task(example_input2);
        assert_eq!(example1, 8);
    }
}
