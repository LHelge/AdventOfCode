use aoc::*;
use std::num::ParseIntError;

struct Report {
    levels: Vec<u64>,
}

impl TryFrom<&str> for Report {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let levels = value
            .split(' ')
            .map(|level| level.parse())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;

        Ok(Self { levels })
    }
}

impl Report {
    fn clone_reduced(&self, index: usize) -> Self {
        let mut levels = self.levels.clone();
        levels.remove(index);

        Self { levels }
    }

    fn safe(&self) -> bool {
        self.levels
            .windows(2)
            .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
            && (self.levels.windows(2).all(|w| w[0] < w[1])
                || self.levels.windows(2).all(|w| w[0] > w[1]))
    }

    fn safe_damped(&self) -> bool {
        if self.safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let report = self.clone_reduced(i);
            if report.safe() {
                return true;
            }
        }

        false
    }
}

fn parse(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(Report::try_from)
        .collect::<Result<Vec<Report>, ParseIntError>>()
        .unwrap()
}

fn task1(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.safe()).count()
}

fn task2(reports: &[Report]) -> usize {
    reports.iter().filter(|r| r.safe_damped()).count()
}

fn main() -> Result<(), ParseIntError> {
    let mut solution = Solution::<usize, Report>::new(2024, 2, &parse, &task1, &task2);
    solution.solve_live();

    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod y2024d02 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let mut solution = Solution::<usize, Report>::new(2024, 2, &parse, &task1, &task2);
        solution.solve(input);
        assert_eq!(solution.task1().unwrap(), 2);
        assert_eq!(solution.task2().unwrap(), 4);
    }
}
