use std::{ops::RangeInclusive, str::FromStr};

use aoc::{AoCError, AoCInput};

#[derive(Debug)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
}

impl FromStr for Instruction {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, rest) = if s.starts_with("turn on ") {
            (
                Action::TurnOn,
                s.trim_start_matches("turn on ").trim_start(),
            )
        } else if s.starts_with("turn off ") {
            (
                Action::TurnOff,
                s.trim_start_matches("turn off ").trim_start(),
            )
        } else if s.starts_with("toggle ") {
            (Action::Toggle, s.trim_start_matches("toggle ").trim_start())
        } else {
            return Err(AoCError::BadInput);
        };

        let (from, to) = rest.split_once(" through ").ok_or(AoCError::BadInput)?;
        let (from_x, from_y) = from.split_once(',').ok_or(AoCError::BadInput)?;
        let (to_x, to_y) = to.split_once(',').ok_or(AoCError::BadInput)?;

        Ok(Self {
            action,
            x: from_x.parse()?..=to_x.parse()?,
            y: from_y.parse()?..=to_y.parse()?,
        })
    }
}

struct LightGrid {
    lights: Vec<Vec<bool>>,
}

impl LightGrid {
    fn new() -> Self {
        Self {
            lights: vec![vec![false; 1000]; 1000],
        }
    }

    fn perform_instruction(&mut self, instruction: &Instruction) {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                match instruction.action {
                    Action::TurnOn => self.lights[x][y] = true,
                    Action::TurnOff => self.lights[x][y] = false,
                    Action::Toggle => self.lights[x][y] = !self.lights[x][y],
                }
            }
        }
    }

    fn count_lit(&self) -> usize {
        self.lights.iter().flatten().filter(|&&l| l).count()
    }
}

struct BrightnessGrid {
    lights: Vec<Vec<usize>>,
}

impl BrightnessGrid {
    fn new() -> Self {
        Self {
            lights: vec![vec![0; 1000]; 1000],
        }
    }

    fn perform_instruction(&mut self, instruction: &Instruction) {
        for x in instruction.x.clone() {
            for y in instruction.y.clone() {
                match instruction.action {
                    Action::TurnOn => self.lights[x][y] += 1,
                    Action::TurnOff => self.lights[x][y] = self.lights[x][y].saturating_sub(1),
                    Action::Toggle => self.lights[x][y] += 2,
                }
            }
        }
    }

    fn count_lit(&self) -> usize {
        self.lights.iter().flatten().sum()
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let instructions = input
        .trim()
        .lines()
        .map(|line| line.trim().parse().expect("Bad input"))
        .collect::<Vec<Instruction>>();

    let mut light_grid = LightGrid::new();
    let mut brightness_grid = BrightnessGrid::new();
    for instruction in instructions.iter() {
        light_grid.perform_instruction(instruction);
        brightness_grid.perform_instruction(instruction);
    }

    let task1 = light_grid.count_lit();
    let task2 = brightness_grid.count_lit();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2015, 6)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d06 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, example2) = solve_task("turn on 0,0 through 999,999");
        assert_eq!(example1, 1000000);
        assert_eq!(example2, 1000000);

        let (example1, example2) = solve_task("toggle 0,0 through 999,0");
        assert_eq!(example1, 1000);
        assert_eq!(example2, 2000);

        let (example1, example2) =
            solve_task("turn on 0,0 through 999,999\nturn off 499,499 through 500,500");
        assert_eq!(example1, 999996);
        assert_eq!(example2, 999996);

        let (example1, example2) = solve_task("turn on 0,0 through 0,0");
        assert_eq!(example1, 1);
        assert_eq!(example2, 1);

        let (example1, example2) = solve_task("toggle 0,0 through 999,999");
        assert_eq!(example1, 1000000);
        assert_eq!(example2, 2000000);
    }
}
