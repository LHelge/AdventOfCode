use aoc::*;

struct Report {
    levels: Vec<u64>
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Self { levels: value.split(' ').map(|level| level.parse().unwrap()).collect() }
    }
}

impl Report {
    fn clone_reduced(&self, index: usize) -> Self {
        let mut levels = self.levels.clone();
        levels.remove(index);

        Self {
            levels
        }
    }

    fn safe(&self) -> bool {
        self.levels.windows(2).all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3) &&
        (self.levels.windows(2).all(|w| w[0] < w[1]) ||
        self.levels.windows(2).all(|w| w[0] > w[1]))
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


fn solve_task(input: &str) -> (usize, usize) {
    let reports = input.lines().map(Report::from).collect::<Vec<Report>>();

    let task1 = reports.iter().filter(|r| r.safe()).count();
    let task2 = reports.iter().filter(|r| r.safe_damped()).count();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2024, 2)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
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

        // Task 1
        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 2);
        assert_eq!(example2, 4);
    }
}
