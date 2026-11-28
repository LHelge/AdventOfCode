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
    parse: Duration,
    task1: SolveResult<T>,
    task2: SolveResult<U>,
}

impl<T: Display, U: Display> Display for Solution<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Advent of Code {} solution", self.date)?;
        writeln!(f, "--------------------------------")?;
        writeln!(f, "Parse: ({} s)", self.parse.as_secs_f64())?;
        writeln!(f, "Task 1: {}", self.task1)?;
        writeln!(f, "Task 2: {}", self.task2)
    }
}

pub trait AoCProblem: Sized {
    type Solution1: Display + Debug + PartialEq;
    type Solution2: Display + Debug + PartialEq;

    const YEAR: u16;
    const DAY: u8;

    fn parse(input: &str) -> Result<Self>;
    fn part1(&self) -> Result<Self::Solution1>;
    fn part2(&self) -> Result<Self::Solution2> {
        Err(AoCError::Unsolved)
    }

    fn date() -> Date {
        Date::new(Self::YEAR, Self::DAY).expect("Invalid YEAR/DAY on AoCProblem impl")
    }

    fn solve_input(input: &str) -> Result<Solution<Self::Solution1, Self::Solution2>> {
        let start = Instant::now();
        let problem = Self::parse(input)?;
        let parse = start.elapsed();

        Ok(Solution {
            date: Self::date(),
            parse,
            task1: timed(|| problem.part1()),
            task2: timed(|| problem.part2()),
        })
    }

    fn solve() -> Result<Solution<Self::Solution1, Self::Solution2>> {
        dotenvy::dotenv().ok();
        let cache = Cache::new("./.cache", env::var("SESSION")?)?;
        let input = cache.get(Self::date())?;
        Self::solve_input(&input)
    }

    fn run() -> Result<()> {
        println!("{}", Self::solve()?);
        Ok(())
    }
}

fn timed<T: Display>(f: impl FnOnce() -> Result<T>) -> SolveResult<T> {
    let start = Instant::now();
    match f() {
        Ok(result) => SolveResult::Solved(result, start.elapsed()),
        Err(AoCError::Unsolved) => SolveResult::Unsolved,
        Err(e) => SolveResult::Error(e),
    }
}
