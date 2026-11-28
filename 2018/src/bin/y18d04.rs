use aoc::{problem::*, utils::*, *};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, multispace1, u8, u16, u64},
    combinator::value,
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
};
use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
    ops::Sub,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct TimeStamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Sub<TimeStamp> for TimeStamp {
    type Output = u8;
    fn sub(self, rhs: TimeStamp) -> Self::Output {
        self.minute.abs_diff(rhs.minute)
    }
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{}-{} {}:{}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    Wake,
    Sleep,
    Begin(u64),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    time: TimeStamp,
    _type: EventType,
}

fn date(input: &str) -> IResult<&str, (u16, u8, u8)> {
    (u16, preceded(char('-'), u8), preceded(char('-'), u8)).parse(input)
}

fn time(input: &str) -> IResult<&str, (u8, u8)> {
    separated_pair(u8, tag(":"), u8).parse(input)
}

fn time_stamp(input: &str) -> IResult<&str, TimeStamp> {
    let (input, ((year, month, day), (hour, minute))) =
        separated_pair(date, char(' '), time).parse(input)?;

    Ok((
        input,
        TimeStamp {
            year,
            month,
            day,
            hour,
            minute,
        },
    ))
}

fn event_type(input: &str) -> IResult<&str, EventType> {
    alt((
        value(EventType::Wake, tag("wakes up")),
        value(EventType::Sleep, tag("falls asleep")),
        delimited(tag("Guard #"), u64, tag(" begins shift")).map(EventType::Begin),
    ))
    .parse(input)
}

fn event(input: &str) -> IResult<&str, Event> {
    let (input, time) = delimited(tag("["), time_stamp, tag("]")).parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _type) = event_type(input)?;

    Ok((input, Event { time, _type }))
}

#[derive(Default)]
struct Problem {
    events: BTreeSet<Event>,
}

impl AoCProblem<u64, u64> for Problem {
    fn date() -> Date {
        Date::new(2018, 4).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        let (_, events) = separated_list1(line_ending, event).parse(input).unwrap();

        self.events = events.into_iter().collect();

        Ok(())
    }

    fn part1(&self) -> Result<u64> {
        let mut asleep: HashMap<u64, u64> = HashMap::new();

        let mut guard: Option<u64> = None;
        let mut start: Option<TimeStamp> = None;
        for event in self.events.iter() {
            match event._type {
                EventType::Begin(id) => guard = Some(id),
                EventType::Sleep => start = Some(event.time),
                EventType::Wake => {
                    let a = asleep.entry(guard.unwrap()).or_default();
                    *a += (event.time - start.unwrap()) as u64;
                }
            }
        }

        dbg!(&asleep);

        Err(AoCError::Unsolved)
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
        let input = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(240);
    }
}
