const YEAR: u16 = 2024;
const DAY: u8 = 13;
use std::fmt::Display;

use aoc::*;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::{many1, separated_list1},
    IResult,
};

trait IsAlmostInt {
    fn is_almost_int(&self) -> bool;
}

impl IsAlmostInt for f64 {
    fn is_almost_int(&self) -> bool {
        const EPSILON: f64 = 1e-2;

        (self - self.round()).abs() < EPSILON
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: u64,
    y: u64,
}

type Distance = Position;

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: Distance,
    button_b: Distance,
    prize: Position,
}

impl ClawMachine {
    fn tokens_to_win(&self) -> Option<u64> {
        let (xa, xb) = (self.button_a.x as f64, self.button_b.x as f64);
        let (ya, yb) = (self.button_a.y as f64, self.button_b.y as f64);
        let (xp, yp) = (self.prize.x as f64, self.prize.y as f64);

        let b = (xp - xa * yp / ya) / (xb - xa * yb / ya);
        let a = (yp - yb * b) / ya;

        if a.is_almost_int() && b.is_almost_int() {
            let a = a.round() as u64;
            let b = b.round() as u64;
            Some(3 * a + b)
        } else {
            None
        }
    }
}

fn distance(input: &str) -> IResult<&str, Distance> {
    let (input, _) = tag("X+")(input)?;
    let (input, x) = nom::character::complete::u64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = nom::character::complete::u64(input)?;

    Ok((input, Distance { x, y }))
}

fn position(input: &str) -> IResult<&str, Position> {
    let (input, _) = tag("X=")(input)?;
    let (input, x) = nom::character::complete::u64(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = nom::character::complete::u64(input)?;

    Ok((input, Position { x, y }))
}

fn button<'a>(input: &'a str, name: &str) -> IResult<&'a str, Distance> {
    let (input, _) = tag(name)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, distance) = distance(input)?;

    Ok((input, distance))
}

fn prize(input: &str) -> IResult<&str, Position> {
    let (input, _) = tag("Prize: ")(input)?;
    let (input, position) = position(input)?;

    Ok((input, position))
}

fn claw_machine(input: &str) -> IResult<&str, ClawMachine> {
    let (input, button_a) = button(input, "Button A")?;
    let (input, _) = newline(input)?;
    let (input, button_b) = button(input, "Button B")?;
    let (input, _) = newline(input)?;
    let (input, prize) = prize(input)?;

    Ok((
        input,
        ClawMachine {
            button_a,
            button_b,
            prize,
        },
    ))
}

type ResultType = u64;
type DataType = Vec<ClawMachine>;

fn parse(input: &str) -> Result<DataType> {
    let (_, machines) =
        separated_list1(many1(newline), claw_machine)(input).map_err(|_| AoCError::BadInput)?;
    Ok(machines)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().filter_map(ClawMachine::tokens_to_win).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let mut data = data.clone();

    Ok(data
        .iter_mut()
        .filter_map(|cm| {
            cm.prize.x += 10000000000000;
            cm.prize.y += 10000000000000;
            cm.tokens_to_win()
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
        let input = r#"Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(480));
        assert_eq!(task2, Some(875318608908));
    }
}
