use std::str::FromStr;

use aoc::{AoCError, AoCInput};

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let amount = parts[1].parse()?;
        let from = parts[3].parse::<usize>()? - 1;
        let to = parts[5].parse::<usize>()? - 1;

        Ok(Move { amount, from, to })
    }
}

fn solve_task(input: &str) -> (String, String) {
    // Split the input into stack initialization and moves
    let mut parts = input.split("\n\n");
    let stack_input = parts.next().expect("Bad input");
    let move_input = parts.next().expect("Bad input");

    // Parse the stack initialization
    let mut stack_input = stack_input.lines().rev();
    let num_stacks = stack_input
        .next()
        .expect("Bad input")
        .split_whitespace()
        .count();
    let mut stacks = vec![vec![]; num_stacks];
    for line in stack_input {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c.is_alphabetic() {
                stacks[i].push(c);
            }
        }
    }

    // Parse the moves
    let moves: Vec<Move> = move_input
        .lines()
        .map(|line| line.trim().parse().expect("Bad input"))
        .collect();

    // Solve the task1 using CrateMover 9000
    let mut stacks1 = stacks.clone();
    for m in &moves {
        for _ in 0..m.amount {
            let c = stacks1[m.from].pop().expect("Bad input");
            stacks1[m.to].push(c);
        }
    }

    // Solve the task2 using CrateMover 9001
    let mut stacks2 = stacks.clone();
    for m in &moves {
        let mut temp = vec![];
        for _ in 0..m.amount {
            temp.push(stacks2[m.from].pop().expect("Bad input"));
        }

        for &c in temp.iter().rev() {
            stacks2[m.to].push(c);
        }
    }

    // Collect the crates from the top of the stacks
    let mut task1 = String::new();
    let mut task2 = String::new();
    for i in 0..num_stacks {
        task1.push(stacks1[i].pop().expect("Bad input"));
        task2.push(stacks2[i].pop().expect("Bad input"));
    }

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2022, 5)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d05 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, "CMZ");
        assert_eq!(example2, "MCD");
    }
}
