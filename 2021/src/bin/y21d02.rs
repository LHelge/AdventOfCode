use aoc::AoCInput;

enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            return Err(());
        }

        match (parts[0], parts[1]) {
            ("forward", n) => Ok(Direction::Forward(n.parse().unwrap())),
            ("up", n) => Ok(Direction::Up(n.parse().unwrap())),
            ("down", n) => Ok(Direction::Down(n.parse().unwrap())),
            _ => Err(()),
        }
    }
}

pub fn distance(input: &str) -> u32 {
    let moves = input
        .lines()
        .map(|l| Direction::try_from(l).unwrap())
        .collect::<Vec<Direction>>();

    let mut depth = 0;
    let mut forward = 0;
    for m in moves {
        match m {
            Direction::Forward(n) => forward += n,
            Direction::Up(n) => depth -= n,
            Direction::Down(n) => depth += n,
        }
    }

    depth * forward
}

pub fn distance_aim(input: &str) -> u32 {
    let moves = input
        .lines()
        .map(|l| Direction::try_from(l).unwrap())
        .collect::<Vec<Direction>>();

    let mut depth = 0;
    let mut forward = 0;
    let mut aim = 0;
    for m in moves {
        match m {
            Direction::Forward(n) => {
                forward += n;
                depth += aim * n;
            }
            Direction::Up(n) => aim -= n,
            Direction::Down(n) => aim += n,
        }
    }

    depth * forward
}

fn solve_task(input: &str) -> (u32, u32) {
    (distance(input), distance_aim(input))
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2021, 2)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d02 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 150);
        assert_eq!(example2, 900);
    }
}
