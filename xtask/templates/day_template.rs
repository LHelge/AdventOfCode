use aoc::{problem::*, utils::*, *};

struct Problem {}

impl AoCProblem for Problem {
    type Solution1 = u64;
    type Solution2 = u64;

    const YEAR: u16 = {{ year }};
    const DAY: u8 = {{ day }};

    fn parse(input: &str) -> Result<Self> {
        Err(AoCError::Unsolved)
    }

    fn part1(&self) -> Result<u64> {
        Err(AoCError::Unsolved)
    }
}

fn main() -> Result<()> {
    Problem::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<()> {
        let input = r#""#;

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part1()?, 0);

        Ok(())
    }
}
