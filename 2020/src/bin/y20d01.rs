use aoc::*;
use std::collections::BTreeSet;

struct Y2020D01 {
    expenses: BTreeSet<u64>,
}

impl CodeProblem for Y2020D01 {
    fn date() -> (u16, u8) {
        (2020, 1)
    }

    fn from_input(input: &str) -> Result<Self> {
        let expenses = input
            .lines()
            .map(|l| l.trim().parse().expect("Bad input"))
            .collect();

        Ok(Self { expenses })
    }

    fn task1(&self) -> Result<u64> {
        let mut task1 = 0;
        for expense in &self.expenses {
            let target = 2020 - expense;
            if self.expenses.contains(&target) {
                task1 = expense * target;
                break;
            }
        }
        Ok(task1)
    }

    fn task2(&self) -> Result<u64> {
        let mut task2 = None;
        for expense1 in &self.expenses {
            for expense2 in &self.expenses {
                if expense1 == expense2 || expense1 + expense2 > 2020 {
                    continue;
                }

                let target = 2020 - expense1 - expense2;
                if self.expenses.contains(&target) {
                    task2 = Some(expense1 * expense2 * target);
                    break;
                }
            }

            if task2.is_some() {
                break;
            }
        }
        task2.ok_or(AoCError::Unsolved)
    }
}

fn main() -> Result<()> {
    Y2020D01::solve()
}

#[cfg(test)]
mod y2020d01 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"1721
979
366
299
675
1456"#;

        let problem = Y2020D01::from_input(input).unwrap();
        assert_eq!(problem.task1().unwrap(), 514579);
        assert_eq!(problem.task2().unwrap(), 241861950);
    }
}
