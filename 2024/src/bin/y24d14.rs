const YEAR: u16 = 2024;
const DAY: u8 = 14;

use aoc::*;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

struct Room {
    width: i64,
    height: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Velocity {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

fn position(input: &str) -> IResult<&str, Position> {
    let (input, (x, y)) = preceded(
        tag("p="),
        separated_pair(complete::i64, tag(","), complete::i64),
    )(input)?;
    Ok((input, Position { x, y }))
}

fn velocity(input: &str) -> IResult<&str, Velocity> {
    let (input, (x, y)) = preceded(
        tag("v="),
        separated_pair(complete::i64, tag(","), complete::i64),
    )(input)?;
    Ok((input, Velocity { x, y }))
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) = separated_pair(position, space1, velocity)(input)?;
    Ok((input, Robot { position, velocity }))
}

impl Robot {
    fn move_seconds(&mut self, seconds: i64, room: &Room) {
        let x = self.position.x + self.velocity.x * seconds;
        let y = self.position.y + self.velocity.y * seconds;

        self.position.x = x.rem_euclid(room.width);
        self.position.y = y.rem_euclid(room.height);
    }
}

#[allow(dead_code)]
fn debug_robots(robots: &[Robot], room: &Room) {
    for y in 0..room.height {
        for x in 0..room.width {
            match robots
                .iter()
                .filter(|robot| robot.position.x == x && robot.position.y == y)
                .count()
            {
                0 => print!("."),
                n => print!("{n}"),
            }
        }
        println!();
    }
}

type ResultType = usize;
type DataType = (Room, Vec<Robot>);

fn parse(input: &str) -> Result<DataType> {
    let (_, robots) = separated_list1(newline, robot)(input).map_err(|_| AoCError::BadInput)?;

    // Ugly hack, but room is smaller for tests
    let room = if cfg!(test) {
        Room {
            width: 11,
            height: 7,
        }
    } else {
        Room {
            width: 101,
            height: 103,
        }
    };

    Ok((room, robots))
}

fn task1(data: &DataType) -> Result<ResultType> {
    let room = &data.0;
    let mut robots = data.1.clone();

    robots
        .iter_mut()
        .for_each(|robot| robot.move_seconds(100, room));

    let q1 = robots
        .iter()
        .filter(|robot| robot.position.x < room.width / 2 && robot.position.y < room.height / 2)
        .count();
    let q2 = robots
        .iter()
        .filter(|robot| robot.position.x > room.width / 2 && robot.position.y < room.height / 2)
        .count();
    let q3 = robots
        .iter()
        .filter(|robot| robot.position.x < room.width / 2 && robot.position.y > room.height / 2)
        .count();
    let q4 = robots
        .iter()
        .filter(|robot| robot.position.x > room.width / 2 && robot.position.y > room.height / 2)
        .count();
    Ok(q1 * q2 * q3 * q4)
}

fn task2(data: &DataType) -> Result<ResultType> {
    let room = &data.0;
    let mut robots = data.1.clone();

    let mut seconds = 0;
    loop {
        seconds += 1;
        robots
            .iter_mut()
            .for_each(|robot| robot.move_seconds(1, room));

        if robots.iter().map(|robot| robot.position).all_unique() {
            break;
        }
    }

    Ok(seconds)
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
        let input = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(12));
        assert_eq!(task2, Some(1));
    }
}
