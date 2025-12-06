use aoc::{problem::*, utils::*, *};

#[derive(Default)]
struct Problem {}

impl AoCProblem<u64, u64> for Problem {
    fn date() -> Date {
        Date::new({{ year }}, {{ day }}).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        Err(AoCError::Unsolved)
    }

    fn part1(&self) -> Result<u64> {
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
        let input = r#""#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(0);
    }
}
