use std::collections::HashMap;

use aoc::AoCInput;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0},
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

type Point = (u64, u64);
type Line = (Point, Point);

fn point(input: &str) -> IResult<&str, Point> {
    separated_pair(
        nom::character::complete::u64,
        nom::character::complete::char(','),
        nom::character::complete::u64,
    )(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    let (input, _) = multispace0(input)?;
    separated_pair(point, tag(" -> "), point)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list0(line_ending, line)(input)
}

#[allow(clippy::reversed_empty_ranges)]
fn solve_task(input: &str) -> (u64, u64) {
    let (_, lines) = parse_lines(input).expect("Bad input");

    let mut points: HashMap<Point, usize> = HashMap::new();
    for line in &lines {
        let (x_range, y_range) = match *line {
            ((x1, y1), (x2, y2)) if x1 == x2 && y1 <= y2 => (x1..=x2, y1..=y2),
            ((x1, y1), (x2, y2)) if x1 == x2 && y1 > y2 => (x1..=x2, y2..=y1),
            ((x1, y1), (x2, y2)) if y1 == y2 && x1 <= x2 => (x1..=x2, y1..=y2),
            ((x1, y1), (x2, y2)) if y1 == y2 && x1 > x2 => (x2..=x1, y1..=y2),
            _ => (1..=0, 1..=0),
        };

        for x in x_range {
            for y in y_range.clone() {
                points.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    let task1 = points.values().filter(|&&v| v > 1).count() as u64;

    for line in &lines {
        let (x_dir, y_dir, num): (i64, i64, u64) = match *line {
            ((x1, y1), (x2, y2)) if x1 < x2 && y1 < y2 => (1, 1, x1.abs_diff(x2) + 1),
            ((x1, y1), (x2, y2)) if x1 < x2 && y1 > y2 => (1, -1, x1.abs_diff(x2) + 1),
            ((x1, y1), (x2, y2)) if x1 > x2 && y1 < y2 => (-1, 1, x1.abs_diff(x2) + 1),
            ((x1, y1), (x2, y2)) if x1 > x2 && y1 > y2 => (-1, -1, x1.abs_diff(x2) + 1),
            _ => (0, 0, 0),
        };

        let (mut x, mut y) = (line.0 .0 as i64, line.0 .1 as i64);
        for _ in 0..num {
            points
                .entry((x as u64, y as u64))
                .and_modify(|e| *e += 1)
                .or_insert(1);
            x += x_dir;
            y += y_dir;
        }
    }

    let task2 = points.values().filter(|&&v| v > 1).count() as u64;

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2021, 5)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d05 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 5);
        assert_eq!(example2, 12);
    }
}
