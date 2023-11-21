struct Sections {
    from: usize,
    to: usize,
}

impl Sections {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split('-').collect();
        if parts.len() == 2 {
            return Self {
                from: parts[0].parse().unwrap_or(0),
                to: parts[1].parse().unwrap_or(0),
            };
        }
        Self { from: 0, to: 0 }
    }

    fn contains(&self, other: &Sections) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlap(&self, other: &Sections) -> bool {
        (self.from >= other.from && self.from <= other.to)
            || (self.to >= other.from && self.to <= other.to)
            || self.contains(other)
    }
}

fn solve_task(input: &str) -> (u64, u64) {
    let mut result1 = 0;
    let mut result2 = 0;
    for line in input.lines() {
        let pairs: Vec<&str> = line.trim().split(',').collect();

        let first = Sections::new(pairs[0]);
        let second = Sections::new(pairs[1]);

        if first.contains(&second) || second.contains(&first) {
            result1 += 1;
        }

        if first.overlap(&second) {
            result2 += 1;
        }

        // Do stuff per line
    }

    (result1, result2)
}

fn main() {
    let input = aoc_input::get_input(
        2022,
        4,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d04 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 2);
        assert_eq!(example2, 4);
    }
}
