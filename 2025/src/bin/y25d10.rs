use std::{collections::HashSet, str::FromStr};

use aoc::{problem::*, utils::*, *};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lights(Vec<bool>);

impl FromStr for Lights {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let lights = s[1..(s.len() - 1)]
            .chars()
            .map(|c| match c {
                '.' => false,
                '#' => true,
                _ => panic!("HEEELP!"),
            })
            .collect();

        Ok(Lights(lights))
    }
}

impl Lights {
    fn new(len: usize) -> Self {
        Lights(vec![false; len])
    }

    fn push(&self, buttons: &Vec<usize>) -> Self {
        let mut new = self.clone();

        for button in buttons {
            new.0[*button] = !new.0[*button];
        }

        new
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Joltages(Vec<u64>);

impl FromStr for Joltages {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let joltages = s[1..(s.len() - 1)]
            .split(',')
            .map(|j| j.parse().unwrap())
            .collect();

        Ok(Joltages(joltages))
    }
}

impl Joltages {
    fn new(len: usize) -> Self {
        Joltages(vec![0; len])
    }

    fn push(&self, buttons: &Vec<usize>) -> Self {
        let mut new = self.clone();

        for button in buttons {
            new.0[*button] += 1;
        }

        new
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
struct Machine {
    lights: Lights,
    buttons: Vec<Vec<usize>>,
    joltages: Joltages,
}

impl FromStr for Machine {
    type Err = AoCError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut buttons = Vec::new();
        let mut joltages = Joltages::new(1);

        let (lights, rest) = s.split_once(' ').ok_or(AoCError::BadInput)?;
        let lights = lights.parse()?;

        for part in rest.split_whitespace() {
            let middle = &part[1..(part.len() - 1)];
            if part.starts_with('(') {
                buttons.push(middle.parse_delimited(',')?);
            } else if part.starts_with('{') {
                joltages = part.parse()?;
            }
        }

        Ok(Machine {
            lights,
            buttons,
            joltages,
        })
    }
}

#[derive(Default)]
struct Problem {
    machines: Vec<Machine>,
}

impl Machine {
    fn least_button_presses(&self) -> usize {
        let mut presses = 0;

        let mut states = HashSet::new();
        states.insert(Lights::new(self.lights.len()));

        loop {
            presses += 1;

            let mut new = Vec::new();
            for state in states.iter() {
                for buttons in self.buttons.iter() {
                    let state = state.push(buttons);
                    if state == self.lights {
                        return presses;
                    }
                    new.push(state);
                }
            }

            states.extend(new);
        }
    }

    fn least_button_presses2(&self) -> usize {
        let mut presses = 0;

        let mut states = HashSet::new();
        states.insert(Joltages::new(self.joltages.len()));

        print!("Looking for presses for {self:?}: ");

        loop {
            presses += 1;

            let mut new = Vec::new();
            for state in states.iter() {
                for buttons in self.buttons.iter() {
                    let state = state.push(buttons);
                    if state == self.joltages {
                        println!("{presses}");
                        return presses;
                    }
                    new.push(state);
                }
            }

            states.extend(new);
        }
    }
}

impl AoCProblem<usize, usize> for Problem {
    fn date() -> Date {
        Date::new(2025, 10).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.machines = input.parse_lines()?;
        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        Ok(self.machines.iter().map(|m| m.least_button_presses()).sum())
    }

    fn part2(&self) -> Result<usize> {
        Ok(self
            .machines
            .iter()
            .map(|m| m.least_button_presses2())
            .sum())
    }
}

fn main() -> Result<()> {
    let mut problem = Problem::default();
    let solution = problem.solve()?;

    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(7);
        problem.test_part2(33);
    }
}
