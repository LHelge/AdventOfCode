use aoc::AoCInput;
use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl From<char> for Turn {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Solve {
    Task1,
    Task2,
    Both,
}

fn solve_task(input: &str, solve: Solve) -> (u64, u64) {
    let mut task1 = 0;
    let mut task2 = 0;

    // Read input
    let mut lines = input.lines();
    let turns = lines
        .next()
        .unwrap()
        .chars()
        .map(Turn::from)
        .collect::<Vec<_>>();

    // Skip one line
    lines.next();

    let nodes: HashMap<&str, (&str, &str)> = lines
        .map(|line| {
            let (name, rest) = line.trim().split_once(" = ").unwrap();
            let (left, right) = rest[1..rest.len() - 1].split_once(", ").unwrap();
            (name, (left, right))
        })
        .collect();

    //println!("Turns: {:?}", &turns);
    //println!("Nodes: {:?}", &nodes);

    // Solve task 1
    if solve == Solve::Task1 || solve == Solve::Both {
        let mut current_node = "AAA";
        for turn in turns.iter().cycle() {
            let (left, right) = nodes.get(current_node).unwrap();
            match turn {
                Turn::Left => current_node = left,
                Turn::Right => current_node = right,
            }
            task1 += 1;

            if current_node == "ZZZ" {
                break;
            }
        }
    }

    // Solve task 2
    if solve == Solve::Task2 || solve == Solve::Both {
        task2 = 1; // for LCM to work
        for start in nodes.iter().filter_map(|(&name, _)| {
            if name.ends_with('A') {
                Some(name)
            } else {
                None
            }
        }) {
            let mut directions = turns.iter().cycle();
            let mut current_node = start;

            let mut steps = 0;
            while !current_node.ends_with('Z') {
                let (left, right) = nodes.get(current_node).unwrap();
                current_node = match directions.next().unwrap() {
                    Turn::Left => left,
                    Turn::Right => right,
                };
                steps += 1;
            }
            task2 = lcm(task2, steps);
        }
    }

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2023, 8)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input, Solve::Both);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d08 {
    use super::*;

    #[test]
    fn example1() {
        let example_input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let (example1, _) = solve_task(example_input, Solve::Task1);
        assert_eq!(example1, 2);
    }

    #[test]
    fn example2() {
        let example_input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let (_, example2) = solve_task(example_input, Solve::Task2);
        assert_eq!(example2, 6);
    }
}
