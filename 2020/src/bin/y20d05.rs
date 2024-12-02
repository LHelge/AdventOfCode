use aoc::AoCInput;

enum FrontBack {
    Front,
    Back,
}

impl From<char> for FrontBack {
    fn from(c: char) -> Self {
        match c {
            'F' => FrontBack::Front,
            'B' => FrontBack::Back,
            _ => panic!("Invalid FrontBack char"),
        }
    }
}

enum LeftRight {
    Left,
    Right,
}

impl From<char> for LeftRight {
    fn from(c: char) -> Self {
        match c {
            'L' => LeftRight::Left,
            'R' => LeftRight::Right,
            _ => panic!("Invalid LeftRight char"),
        }
    }
}

fn calc_pos(input: &str) -> usize {
    let rows = input
        .chars()
        .take(7)
        .map(FrontBack::from)
        .collect::<Vec<_>>();
    let cols = input
        .chars()
        .skip(7)
        .map(LeftRight::from)
        .collect::<Vec<_>>();

    let row = rows.iter().fold(0usize, |acc, fb| {
        (acc << 1)
            | match fb {
                FrontBack::Front => 0,
                FrontBack::Back => 1,
            }
    });
    let col = cols.iter().fold(0usize, |acc, c| {
        (acc << 1)
            | match c {
                LeftRight::Left => 0,
                LeftRight::Right => 1,
            }
    });

    row * 8 + col
}

fn solve_task(input: &str) -> (usize, usize) {
    let mut seats = input.lines().map(calc_pos).collect::<Vec<_>>();
    seats.sort_unstable();
    seats.dedup();

    let task1 = *seats.iter().last().unwrap();

    let task2 = seats
        .windows(3)
        .find_map(|w| {
            if w[0] + 1 != w[1] {
                Some(w[0] + 1)
            } else if w[1] + 1 != w[2] {
                Some(w[1] + 1)
            } else {
                None
            }
        })
        .unwrap_or_default();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2020, 5)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2020d05 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task("BFFFBBFRRR");
        assert_eq!(example1, 567);

        let (example2, _) = solve_task("FFFBBBFRRR");
        assert_eq!(example2, 119);

        let (example3, _) = solve_task("BBFFBBFRLL");
        assert_eq!(example3, 820);
    }
}
