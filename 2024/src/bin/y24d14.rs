const YEAR: u16 = 2024;
const DAY: u8 = 14;
use aoc::*;
use nom::{
    bytes::complete::tag,
    character::{
        self,
        complete::{newline, space1},
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

struct Room {
    width: u64,
    height: u64,
}

#[derive(Debug, Clone)]
struct Position {
    x: u64,
    y: u64,
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
    let (input, _) = tag("p=")(input)?;
    let (input, (x, y)) =
        separated_pair(character::complete::u64, tag(","), character::complete::u64)(input)?;
    Ok((input, Position { x, y }))
}

fn velocity(input: &str) -> IResult<&str, Velocity> {
    let (input, _) = tag("v=")(input)?;
    let (input, (x, y)) =
        separated_pair(character::complete::i64, tag(","), character::complete::i64)(input)?;
    Ok((input, Velocity { x, y }))
}

fn robot(input: &str) -> IResult<&str, Robot> {
    let (input, (position, velocity)) = separated_pair(position, space1, velocity)(input)?;
    Ok((input, Robot { position, velocity }))
}

impl Robot {
    fn move_robot(&mut self, seconds: u64, room: &Room) {
        let mut x = self.position.x as i64;
        let mut y = self.position.y as i64;

        x += self.velocity.x * seconds as i64;
        y += self.velocity.y * seconds as i64;

        x %= room.width as i64;
        y %= room.height as i64;

        if x < 0 {
            x += room.width as i64;
        }
        if y < 0 {
            y += room.height as i64;
        }

        self.position.x = x as u64;
        self.position.y = y as u64;
    }
}

type ResultType = usize;
type DataType = (Room, Vec<Robot>);

fn parse(input: &str) -> Result<DataType> {
    let (_, robots) = separated_list1(newline, robot)(input).map_err(|_| AoCError::BadInput)?;

    // Ugly hack, but room is smaller for tests
    let room = if robots.len() == 12 {
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
    let mut data = data.1.clone();

    data.iter_mut()
        .for_each(|robot| robot.move_robot(100, room));

    let q1 = data
        .iter()
        .filter(|robot| robot.position.x < room.width / 2 && robot.position.y < room.height / 2)
        .count();
    let q2 = data
        .iter()
        .filter(|robot| robot.position.x > room.width / 2 && robot.position.y < room.height / 2)
        .count();
    let q3 = data
        .iter()
        .filter(|robot| robot.position.x < room.width / 2 && robot.position.y > room.height / 2)
        .count();
    let q4 = data
        .iter()
        .filter(|robot| robot.position.x > room.width / 2 && robot.position.y > room.height / 2)
        .count();
    Ok(q1 * q2 * q3 * q4)
}

fn task2(_data: &DataType) -> Result<ResultType> {
    Err(AoCError::Unsolved)
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
        assert_eq!(task2, None);
    }
}
