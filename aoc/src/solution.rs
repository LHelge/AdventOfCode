use crate::AoCInput;
use crate::error::*;
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug)]
enum SolutionStatus<T: Display> {
    Unsolved,
    Solved(Duration, T),
}

impl<T: Display> Display for SolutionStatus<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsolved => write!(f, "Unsolved"),
            Self::Solved(time, solution) => write!(f, "{} (in {} s)", solution, time.as_secs_f64()),
        }
    }
}

#[derive(Debug)]
enum ParsingStatus {
    Unparsed,
    Parsed(Duration),
}

impl Display for ParsingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unparsed => write!(f, "Unparsed"),
            Self::Parsed(time) => write!(f, "Parsed (in {} s)", time.as_secs_f64()),
        }
    }
}

pub type Parser<R> = dyn Fn(&str) -> Result<R>;
pub type Solver<T, R> = dyn Fn(&R) -> Result<T>;

pub struct Solution<'a, T, R>
where
    T: Display,
    T: Copy,
{
    parser: &'a Parser<R>,
    task1_solver: &'a Solver<T, R>,
    task2_solver: &'a Solver<T, R>,
    parse: ParsingStatus,
    task1: SolutionStatus<T>,
    task2: SolutionStatus<T>,
}

impl<'a, T, R> Solution<'a, T, R>
where
    T: Display,
    T: Copy,
{
    pub fn new(parser: &'a Parser<R>, task1: &'a Solver<T, R>, task2: &'a Solver<T, R>) -> Self {
        Self {
            parser,
            task1_solver: task1,
            task2_solver: task2,
            parse: ParsingStatus::Unparsed,
            task1: SolutionStatus::Unsolved,
            task2: SolutionStatus::Unsolved,
        }
    }

    pub fn solve_for_answer_input(&mut self, input: &str) -> Result<()> {
        // Parse
        let start = Instant::now();
        let data = (self.parser)(input)?;
        self.parse = ParsingStatus::Parsed(start.elapsed());

        // Solve task 1
        let start = Instant::now();
        match (self.task1_solver)(&data) {
            Ok(task1) => self.task1 = SolutionStatus::Solved(start.elapsed(), task1),
            Err(AoCError::Unsolved) => self.task1 = SolutionStatus::Unsolved,
            Err(e) => return Err(e),
        }

        // Solve task 2
        let start = Instant::now();
        match (self.task2_solver)(&data) {
            Ok(task2) => self.task2 = SolutionStatus::Solved(start.elapsed(), task2),
            Err(AoCError::Unsolved) => self.task2 = SolutionStatus::Unsolved,
            Err(e) => return Err(e),
        }

        Ok(())
    }

    pub fn solve_for_answer(&mut self, year: u16, day: u8) -> Result<()> {
        let input = AoCInput::from_env()?.get_input(year, day)?;
        self.solve_for_answer_input(&input)
    }

    pub fn solve_for_test(&mut self, input: &str) -> Result<(Option<T>, Option<T>)> {
        let data = (self.parser)(input)?;
        let task1 = (self.task1_solver)(&data).ok();
        let task2 = (self.task2_solver)(&data).ok();
        Ok((task1, task2))
    }
}

impl<T, R> Display for Solution<'_, T, R>
where
    T: Display,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.parse)?;
        writeln!(f, "Task 1: {}", self.task1)?;
        writeln!(f, "Task 2: {}", self.task2)
    }
}
