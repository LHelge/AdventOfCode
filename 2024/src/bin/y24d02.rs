const YEAR: u16 = 2024;
const DAY: u8 = 2;

use std::num::ParseIntError;

use aoc::*;

struct Report {
    levels: Vec<u64>,
}

impl TryFrom<&str> for Report {
    type Error = AoCError;

    fn try_from(value: &str) -> Result<Self> {
        let levels = value
            .split(' ')
            .map(|level| level.parse())
            .collect::<std::result::Result<Vec<u64>, ParseIntError>>()?;

        Ok(Self { levels })
    }
}

//impl TryFrom<&str> for Report {
//    type Error = AoCError;
//
//    fn try_from(value: &str) -> Result<Self> {
//        let levels = value
//            .split(' ')
//            .map(|level| level.parse().map_err(|e: ParseIntError| e.into()))
//            .collect::<Result<Vec<u64>>>()?;
//
//        Ok(Self { levels })
//    }
//}

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

type ResultType = usize;
type DataType = Vec<Report>;

fn parse(input: &str) -> Result<DataType> {
    input
        .lines()
        .map(Report::try_from)
        .collect::<Result<Vec<Report>>>()
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().filter(|r| r.safe()).count())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().filter(|r| r.safe_damped()).count())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(2));
        assert_eq!(task2, Some(4));
    }
}
