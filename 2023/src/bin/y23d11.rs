use aoc::{pairs::Pairable, AoCInput};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct Position(usize, usize);

impl Position {
    fn distance_to(&self, other: &Position) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn solve_task(input: &str, old_galaxy_factor: usize) -> (usize, usize) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    // List all columns and rows to remove those that have a galaxy
    let mut expand_columns: BTreeSet<usize> = BTreeSet::new();
    let mut expand_rows: BTreeSet<usize> = BTreeSet::new();
    expand_columns.extend(0..width);
    expand_rows.extend(0..height);

    let mut galaxies: Vec<Position> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    // Remove rows and columns containing galaxies
                    expand_rows.remove(&y);
                    expand_columns.remove(&x);

                    // Add galaxy to list
                    galaxies.push(Position(x, y));
                }
                '.' => {}
                _ => panic!("Unknown character: {}", c),
            }
        }
    }
    let mut older_galaxies = galaxies.clone();

    // Expand columns
    for &col in expand_columns.iter().rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 > col {
                galaxy.0 += 1;
            }
        }

        for galaxy in older_galaxies.iter_mut() {
            if galaxy.0 > col {
                galaxy.0 += old_galaxy_factor - 1;
            }
        }
    }

    // Expand rows
    for &row in expand_rows.iter().rev() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 > row {
                galaxy.1 += 1;
            }
        }

        for galaxy in older_galaxies.iter_mut() {
            if galaxy.1 > row {
                galaxy.1 += old_galaxy_factor - 1;
            }
        }
    }

    // Calculate distance between all galaxies
    let task1 = galaxies.pairs().map(|(a, b)| a.distance_to(&b)).sum();
    let task2 = older_galaxies.pairs().map(|(a, b)| a.distance_to(&b)).sum();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2023, 11)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input, 1000000);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d11 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

        let (example1, example2) = solve_task(example_input, 10);
        assert_eq!(example1, 374);
        assert_eq!(example2, 1030);

        let (_, example2) = solve_task(example_input, 100);
        assert_eq!(example2, 8410);
    }
}
