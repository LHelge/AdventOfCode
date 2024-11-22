use std::time::Instant;

use crate::{error::*, AoCInput};

pub trait CodeProblem
where
    Self: Sized,
{
    fn date() -> (u16, u8);
    fn task1(&self) -> Result<u64> {
        Err(AoCError::Unsolved)
    }
    fn task2(&self) -> Result<u64> {
        Err(AoCError::Unsolved)
    }
    fn from_input(input: &str) -> Result<Self>;

    fn solve() -> Result<()> {
        let (year, day) = Self::date();
        let input = AoCInput::from_env().get_input(year, day)?;

        let problem = Self::from_input(&input)?;

        let task1_start = Instant::now();
        let task1 = problem.task1();
        let task1_time = task1_start.elapsed();

        let task2_start = Instant::now();
        let task2 = problem.task2();
        let task2_time = task2_start.elapsed();

        match task1 {
            Ok(solution) => println!(
                "Task 1: {} ({} s)",
                solution,
                task1_time.as_millis() as f64 / 1000.0
            ),
            Err(AoCError::Unsolved) => println!("Task 1: Unsolved"),
            Err(e) => eprintln!("Task 1: failed ({e})"),
        }

        match task2 {
            Ok(solution) => println!(
                "Task 2: {} ({} s)",
                solution,
                task2_time.as_millis() as f64 / 1000.0
            ),
            Err(AoCError::Unsolved) => println!("Task 2: Unsolved"),
            Err(e) => eprintln!("Task 2: failed ({e})"),
        }

        Ok(())
    }
}
