use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let amount = parts[1].parse()?;
        let from = parts[3].parse()?;
        let to = parts[5].parse()?;

        Ok(Move { amount, from, to })
    }
}

fn solve_task(input: &str) -> (String, String) {
    let mut parts = input.split("\n\n");
    let stack_input = parts.next().expect("Bad input");
    let move_input = parts.next().expect("Bad input");

    let mut stack_input = stack_input.lines().rev();
    let num_stacks = stack_input
        .next()
        .expect("Bad input")
        .split_whitespace()
        .count();
    let mut stacks = vec![vec![]; num_stacks];
    for line in stack_input {
        for i in 0..num_stacks {
            if let Some(char) = line.chars().nth(i * 4 + 1) {
                if char.is_alphabetic() {
                    stacks[i].push(char);
                }
            }
        }
    }

    println!("stacks: {:?}", stacks);

    let moves: Vec<Move> = move_input
        .lines()
        .map(|line| line.trim().parse().expect("Bad input"))
        .collect();
    println!("moves: {:?}", moves);

    let mut stacks1 = stacks.clone();
    for m in &moves {
        for _ in 0..m.amount {
            let c = stacks1[m.from - 1].pop().expect("Bad input");
            stacks1[m.to - 1].push(c);
        }
    }

    let mut stacks2 = stacks.clone();
    for m in &moves {
        let mut temp = vec![];
        for _ in 0..m.amount {
            temp.push(stacks2[m.from - 1].pop().expect("Bad input"));
        }

        for c in temp {
            stacks1[m.to - 1].push(c);
        }
    }

    let mut task1 = String::new();
    let mut task2 = String::new();
    for i in 0..num_stacks {
        task1.push(stacks1[i].pop().expect("Bad input"));
        task2.push(stacks2[i].pop().expect("Bad input"));
    }

    (task1, task2)
}

fn main() {
    let input = aoc_input::get_input(
        2023,
        05,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d05 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = "    [D]    \n\
        [N] [C]    \n\
        [Z] [M] [P]\n\
         1   2   3 \n\
        \n\
        move 1 from 2 to 1\n\
        move 3 from 1 to 3\n\
        move 2 from 2 to 1\n\
        move 1 from 1 to 2";

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, "CMZ");
        assert_eq!(example2, "MCD");
    }
}
