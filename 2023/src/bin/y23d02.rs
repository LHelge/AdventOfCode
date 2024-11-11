use aoc::{AoCError, AoCInput};
use std::str::FromStr;

#[derive(Debug, Default)]
struct Round {
    blue: u64,
    green: u64,
    red: u64,
}

impl FromStr for Round {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut round = Self::default();

        for cubes in s.split(',') {
            match cubes.trim().split_once(' ') {
                Some((cubes, "blue")) => round.blue += cubes.parse::<u64>()?,
                Some((cubes, "green")) => round.green += cubes.parse::<u64>()?,
                Some((cubes, "red")) => round.red += cubes.parse::<u64>()?,
                _ => return Err(AoCError::BadInput),
            }
        }

        Ok(round)
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rounds) = s.split_once(':').ok_or(AoCError::BadInput)?;

        let id: u64 = game.split_once(' ').ok_or(AoCError::BadInput)?.1.parse()?;

        let rounds = rounds
            .split(';')
            .map(|r| r.parse().expect("Bad round format"))
            .collect();

        Ok(Self { id, rounds })
    }
}

/// Filter map function for task 1, return game id for all invalid games
fn task1_filter_map(game: &Game) -> Option<u64> {
    if game
        .rounds
        .iter()
        .any(|r| r.red > 12 || r.green > 13 || r.blue > 14)
    {
        None
    } else {
        Some(game.id)
    }
}

/// Map function for task 2, return the product of the smallest number of cubes of each color
/// for the game to be possible
fn task2_map(game: &Game) -> u64 {
    let min_red = game.rounds.iter().map(|r| r.red).max().unwrap_or(0);
    let min_green = game.rounds.iter().map(|r| r.green).max().unwrap_or(0);
    let min_blue = game.rounds.iter().map(|r| r.blue).max().unwrap_or(0);

    min_red * min_green * min_blue
}

fn solve_task(input: &str) -> (u64, u64) {
    let games: Vec<Game> = input
        .lines()
        .map(|l| l.trim().parse().expect("Bad line format"))
        .collect();

    let task1 = games.iter().filter_map(task1_filter_map).sum();
    let task2 = games.iter().map(task2_map).sum();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2023, 2)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d02 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 8);
        assert_eq!(example2, 2286);
    }
}
