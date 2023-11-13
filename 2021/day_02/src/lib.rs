enum Direction {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.trim().split_whitespace().collect::<Vec<&str>>();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_02_example() {
        let input = r#"forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2"#;

        let result = distance(input);
        assert_eq!(result, 150);
    }

    #[test]
    fn day_02_tasks() {
        let input = aoc_input::get_input(
            2021,
            2,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        // Task 1
        let task1 = distance(&input);
        assert_eq!(task1, 1499229);

        // Task 2
        let task2 = distance_aim(&input);
        assert_eq!(task2, 1340836560);
    }
}
