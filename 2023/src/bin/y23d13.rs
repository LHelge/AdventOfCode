const YEAR: u16 = 2023;
const DAY: u8 = 13;

use aoc::{utils::*, *};
use nom::{
    IResult,
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Material {
    Ash,
    Rocks,
}

impl TryFrom<char> for Material {
    type Error = AoCError;

    fn try_from(value: char) -> Result<Self> {
        match value {
            '#' => Ok(Material::Rocks),
            '.' => Ok(Material::Ash),
            _ => Err(AoCError::BadInput),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

#[derive(Debug, Clone)]
struct Pattern {
    original: Vec2d<Material>,
    transposed: Vec2d<Material>,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.original.data() {
            for m in row {
                write!(
                    f,
                    "{}",
                    match m {
                        Material::Ash => '.',
                        Material::Rocks => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Pattern {
    fn new(original: Vec2d<Material>) -> Self {
        let transposed = original.transposed();

        Self {
            original,
            transposed,
        }
    }

    fn find_reflections(&self) -> Vec<Reflection> {
        let mut reflections = Vec::new();
        // Look for horizontal reflection
        for i in 1..self.original.height() {
            let (first, last) = self.original.data().split_at(i);

            if first.iter().rev().zip(last.iter()).all(|(a, b)| a == b) {
                reflections.push(Reflection::Horizontal(i));
            }
        }

        // Look for vertical reflection
        for i in 1..self.transposed.height() {
            let (first, last) = self.transposed.data().split_at(i);

            if first.iter().rev().zip(last.iter()).all(|(a, b)| a == b) {
                reflections.push(Reflection::Vertical(i));
            }
        }

        reflections
    }

    fn find_alternate_reflection(&mut self) -> Option<Reflection> {
        let original = *self.find_reflections().first()?;

        for pos in self.original.size().iter() {
            self.fix_smudge(pos).unwrap();

            for &reflection in self.find_reflections().iter() {
                if reflection != original {
                    // reset smudge
                    self.fix_smudge(pos).unwrap();

                    return Some(reflection);
                }
            }

            // reset smudge
            self.fix_smudge(pos).unwrap();
        }

        None
    }

    fn fix_smudge(&mut self, pos: Position) -> Result<()> {
        self.original.modify(pos, toggle)?;
        self.transposed.modify(pos.invert(), toggle)?;
        Ok(())
    }
}

fn toggle(m: &mut Material) {
    *m = match m {
        Material::Ash => Material::Rocks,
        Material::Rocks => Material::Ash,
    };
}

fn pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, rows) = separated_list1(line_ending, many1(one_of("#.")))(input)?;
    let data = rows
        .iter()
        .map(|row| row.iter().map(|&c| c.try_into().unwrap()).collect())
        .collect::<Vec<Vec<Material>>>();

    let original = Vec2d::new(data).unwrap();

    Ok((input, Pattern::new(original)))
}

type ResultType = usize;
type DataType = Vec<Pattern>;

fn parse(input: &str) -> Result<DataType> {
    let (_, patterns) =
        separated_list1(many1(line_ending), pattern)(input).map_err(|_| AoCError::BadInput)?;
    Ok(patterns)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .map(|p| match p.find_reflections().first() {
            Some(Reflection::Vertical(i)) => *i,
            Some(Reflection::Horizontal(i)) => i * 100,
            _ => panic!("No reflection found in pattern: \n{}", p),
        })
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut data = data.clone();

    Ok(data
        .iter_mut()
        .map(|p| match p.find_alternate_reflection() {
            Some(Reflection::Vertical(i)) => i,
            Some(Reflection::Horizontal(i)) => i * 100,
            _ => panic!("No reflection found in pattern: \n{}", p),
        })
        .sum())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"#.##..##.
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

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(405));
        assert_eq!(task2, Some(400));
    }
}
