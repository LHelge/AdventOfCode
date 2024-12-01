use aoc::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Plane {
    rows: Vec<String>,
    cols: Vec<String>,
}

impl FromStr for Plane {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().map(|l| l.trim().to_owned()).collect::<Vec<_>>();

        let cols = (0..rows[0].len())
            .map(|i| {
                rows.iter()
                    .map(|r| r.chars().nth(i).unwrap())
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        Ok(Self { rows, cols })
    }
}

impl Plane {
    fn is_mirrored_col(&self) -> Option<usize> {
        (1..self.cols.len() - 1).find(|&col| {
            self.cols[col..]
                .iter()
                .zip(self.cols[..col].iter().rev())
                .all(|(a, b)| a == b)
        })
    }

    fn is_mirrored_row(&self) -> Option<usize> {
        (1..self.rows.len() - 1).find(|&row| {
            self.rows[row..]
                .iter()
                .zip(self.rows[..row].iter().rev())
                .all(|(a, b)| a == b)
        })
    }

    fn print_mirrored_row(&self, row: usize) {
        for (i, r) in self.rows.iter().enumerate() {
            if row == i {
                println!("{}", "-".repeat(r.len() + 3));
            }
            println!("{}: {}", i + 1, r);
        }
    }

    fn print_mirrored_col(&self, col: usize) {
        for (i, c) in self.cols.iter().enumerate() {
            if col == i {
                println!("{}", "-".repeat(c.len() + 3));
            }
            println!("{}: {}", i + 1, c);
        }
    }
}

fn solve_task(input: &str) -> (usize, u64) {
    let planes: Vec<Plane> = input
        .split("\n\n")
        .map(|s| s.parse::<Plane>().unwrap())
        .collect();

    let mut task1 = 0;
    for plane in planes {
        if let Some(col) = plane.is_mirrored_col() {
            task1 += col;

            plane.print_mirrored_col(col);
            println!("Col score: {}\n", col);
        }
        if let Some(row) = plane.is_mirrored_row() {
            task1 += 100 * row;

            plane.print_mirrored_row(row);
            println!("Row score: {}\n", 100 * row);
        }
    }

    (task1, 0)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2023, 12)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d13 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

        let (example1, _example2) = solve_task(example_input);

        assert_eq!(example1, 405);
        //assert_eq!(example2, 0);
    }
}
