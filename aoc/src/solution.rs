use std::{
    fmt::Display,
    time::{Duration, Instant},
};

use crate::AoCInput;

#[derive(Debug)]
enum SolutionStatus<T: Display> {
    Unsolved,
    Solving(Instant),
    Solved(Duration, T),
}

impl<T: Display> Display for SolutionStatus<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsolved => write!(f, "Unsolved"),
            Self::Solving(_) => write!(f, "Solving"),
            Self::Solved(time, solution) => write!(f, "{} ({} s)", solution, time.as_secs_f64()),
        }
    }
}

pub type Parser<R> = dyn Fn(&str) -> Vec<R>;
pub type Solver<T, R> = dyn Fn(&[R]) -> T;

pub struct Solution<'a, T,R>
where
    T: Display,
    T: Copy,
{
    year: u16,
    day: u8,
    parser: &'a Parser<R>,
    task1_solver: &'a Solver<T, R>,
    task2_solver: &'a Solver<T, R>,
    parse: SolutionStatus<usize>,
    task1: SolutionStatus<T>,
    task2: SolutionStatus<T>,
}

impl<'a, T, R> Solution<'a, T, R>
where
    T: Display,
    T: Copy,
{
    pub fn new(year: u16, day: u8, parser: &'a Parser<R>, task1: &'a Solver<T,R>, task2: &'a Solver<T, R>) -> Self {
        Self {
            year,
            day,
            parser,
            task1_solver: task1,
            task2_solver: task2,
            parse: SolutionStatus::Unsolved,
            task1: SolutionStatus::Unsolved,
            task2: SolutionStatus::Unsolved,
        }
    }

    pub fn solve(&mut self, input: &str) {
        let data = self.parse(&input);
        self.solve_task1(&data);
        self.solve_task2(&data);
    }

    pub fn solve_live(&mut self) {
        let input = AoCInput::from_env().get_input(self.year, self.day).unwrap();
        self.solve(&input);
    }

    pub fn parse(&mut self, input: &str) -> Vec<R> {
        let start = Instant::now();
        self.parse = SolutionStatus::Solving(start);
        let out = (self.parser)(input);
        self.parse = SolutionStatus::Solved(start.elapsed(), out.len());
        out
    }

    pub fn solve_task1(&mut self, input: &[R]) {
        let start = Instant::now();
        self.task1 = SolutionStatus::Solving(start);
        let solution = (self.task1_solver)(input);
        self.task1 = SolutionStatus::Solved(start.elapsed(), solution);
    }

    pub fn solve_task2(&mut self, input: &[R]) {
        let start = Instant::now();
        self.task2 = SolutionStatus::Solving(start);
        let solution = (self.task2_solver)(input);
        self.task2 = SolutionStatus::Solved(start.elapsed(), solution);
    }

    pub fn task1(&self) -> Option<T> {
        if let SolutionStatus::Solved(_, result) = self.task1 {
            Some(result)
        } else {
            None
        }
    }

    pub fn task2(&self) -> Option<T> {
        if let SolutionStatus::Solved(_, result) = self.task2 {
            Some(result)
        } else {
            None
        }
    }
}

impl<T, R> Display for Solution<'_, T, R>
where
    T: Display,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "AoC {} day {}", self.year, self.day)?;
        writeln!(f, "-------------------")?;
        writeln!(f, "Parsed items: {}", self.parse)?;
        writeln!(f, "Task 1: {}", self.task1)?;
        writeln!(f, "Task 2: {}", self.task2)
    }
}
