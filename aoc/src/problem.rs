use crate::{cache::Cache, error::*};
use std::{
    env,
    fmt::{Debug, Display},
    time::{Duration, Instant},
};

pub struct Date {
    year: u16,
    day: u8,
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} day {}", self.year, self.day)
    }
}

impl Date {
    pub fn new(year: u16, day: u8) -> Result<Self> {
        match (year, day) {
            (2025, 1..=12) => Ok(Self { year, day }),
            (2015..=2024, 1..=25) => Ok(Self { year, day }),
            _ => Err(AoCError::InvalidDay(year, day)),
        }
    }

    pub fn filename(&self) -> String {
        format!("y{:02}d{:02}.txt", self.year % 2000, self.day)
    }

    pub fn url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }
}

pub enum ParseResult {
    Unparsed,
    Parsed(Duration),
    Error(AoCError),
}

impl From<AoCError> for ParseResult {
    fn from(value: AoCError) -> Self {
        ParseResult::Error(value)
    }
}

impl Display for ParseResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseResult::Unparsed => write!(f, "Not parsed yet"),
            ParseResult::Parsed(dur) => write!(f, "({} s)", dur.as_secs_f64()),
            ParseResult::Error(e) => write!(f, "Error: {e}"),
        }
    }
}

pub enum SolveResult<T: Display> {
    Unsolved,
    Solved(T, Duration),
    Error(AoCError),
}

impl<T: Display> From<AoCError> for SolveResult<T> {
    fn from(value: AoCError) -> Self {
        Self::Error(value)
    }
}

impl<T: Display> Display for SolveResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolveResult::Unsolved => write!(f, "Not solved yet"),
            SolveResult::Solved(res, dur) => write!(f, "{res} in ({})", dur.as_secs_f64()),
            SolveResult::Error(e) => write!(f, "Error: {e}"),
        }
    }
}

pub struct Solution<T: Display, U: Display> {
    date: Date,
    parse: ParseResult,
    task1: SolveResult<T>,
    task2: SolveResult<U>,
}

impl<T: Display, U: Display> Display for Solution<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Advent of Code {} solution", self.date)?;
        writeln!(f, "--------------------------------")?;
        writeln!(f, "Parse: {}", self.parse)?;
        writeln!(f, "Task 1: {}", self.task1)?;
        writeln!(f, "Task 2: {}", self.task2)
    }
}

pub trait AoCProblem<T, U>
where
    T: Display + Debug + Eq,
    U: Display + Debug + Eq,
{
    fn date() -> Date;
    fn parse(&mut self, input: &str) -> Result<()>;
    fn part1(&self) -> Result<T>;
    fn part2(&self) -> Result<U> {
        Err(AoCError::Unsolved)
    }

    fn solve_input(&mut self, input: &str) -> Solution<T, U> {
        let start = Instant::now();
        let parse = match self.parse(input) {
            Ok(()) => ParseResult::Parsed(start.elapsed()),
            Err(e) => ParseResult::Error(e),
        };

        let start = Instant::now();
        let task1 = match self.part1() {
            Ok(result) => SolveResult::Solved(result, start.elapsed()),
            Err(e) => SolveResult::Error(e),
        };

        let start = Instant::now();
        let task2 = match self.part2() {
            Ok(result) => SolveResult::Solved(result, start.elapsed()),
            Err(e) => SolveResult::Error(e),
        };

        Solution {
            date: Self::date(),
            parse,
            task1,
            task2,
        }
    }

    fn solve(&mut self) -> Result<Solution<T, U>> {
        dotenvy::dotenv().ok();
        let session = env::var("SESSION").expect("The environment variable SESSION is not set");
        let cache = Cache::new("./.cache", session).unwrap();

        let input = cache.get(Self::date())?;
        Ok(self.solve_input(&input))
    }

    fn test_part1(&mut self, expected: T) {
        let result = self.part1().expect("Failed to solve part 1");
        assert_eq!(result, expected);
    }

    fn test_part2(&mut self, expected: U) {
        let result = self.part2().expect("Failed to solve part 2");
        assert_eq!(result, expected);
    }
}
