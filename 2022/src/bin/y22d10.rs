use std::str::FromStr;

use aoc::{AoCError, AoCInput};

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut instr = s.split_whitespace();

        match instr.next().unwrap() {
            "noop" => Ok(Self::Noop),
            "addx" => Ok(Self::AddX(instr.next().unwrap().parse::<i32>()?)),
            _ => Err(AoCError::BadInput),
        }
    }
}

fn solve_task(input: &str) -> (i32, Vec<i32>) {
    let instructions: Vec<Instruction> = input.lines().map(|i| i.parse().unwrap()).collect();

    // Task 1
    const INTERESTING: [usize; 6] = [20, 60, 100, 140, 180, 220];
    let mut reg_value: Vec<i32> = Vec::new();
    let mut reg = 1;
    for i in instructions.iter().cycle() {
        match i {
            Instruction::Noop => reg_value.push(reg),
            Instruction::AddX(x) => {
                reg_value.push(reg);
                reg_value.push(reg);
                reg += x;
            }
        }

        if reg_value.len() > 240 {
            break;
        }
    }
    let mut task1 = 0;
    for &i in INTERESTING.iter() {
        task1 += i as i32 * reg_value[i - 1];
    }

    (task1, reg_value)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2022, 10)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2:");
    for row in 0..6 {
        for col in 0..40 {
            let reg = task2[40 * row + col];
            if (col as i32) - 1 <= reg && (col as i32) + 1 >= reg {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod y2022d10 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let (example1, _example2) = solve_task(example_input);

        assert_eq!(example1, 13140);
        //assert_eq!(example2, 0);
    }
}
