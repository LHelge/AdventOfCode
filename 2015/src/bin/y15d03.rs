use std::collections::HashSet;

enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::North,
            'v' => Direction::South,
            '>' => Direction::East,
            '<' => Direction::West,
            _ => panic!("Bad character: {}", c),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Position {
    north: i32,
    south: i32,
}

impl Position {
    fn new() -> Position {
        Position { north: 0, south: 0 }
    }

    fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.south += 1,
            Direction::South => self.south -= 1,
            Direction::East => self.north += 1,
            Direction::West => self.north -= 1,
        }
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let moves = input
        .trim()
        .chars()
        .map(Direction::from)
        .collect::<Vec<Direction>>();

    let mut santa_position = Position::new();
    let mut robot_position = Position::new();
    let mut houses: HashSet<Position> = HashSet::new();

    houses.insert(Position::new());
    for direction in moves.iter() {
        santa_position.go(direction);
        houses.insert(santa_position.clone());
    }
    let task1 = houses.len();

    houses.clear();
    santa_position = Position::new();
    houses.insert(Position::new());
    for directions in moves.chunks(2) {
        if let Some(santas_move) = directions.get(0) {
            santa_position.go(santas_move);
            houses.insert(santa_position.clone());
        }
        if let Some(robots_move) = directions.get(1) {
            robot_position.go(robots_move);
            houses.insert(robot_position.clone());
        }
    }
    let task2 = houses.len();

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2015,
        3,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d03 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task(">");
        assert_eq!(example1, 2);

        let (_, example2) = solve_task("^v");
        assert_eq!(example2, 3);

        let (example1, example2) = solve_task("^>v<");
        assert_eq!(example1, 4);
        assert_eq!(example2, 3);

        let (example1, example2) = solve_task("^v^v^v^v^v");
        assert_eq!(example1, 2);
        assert_eq!(example2, 11);
    }
}
