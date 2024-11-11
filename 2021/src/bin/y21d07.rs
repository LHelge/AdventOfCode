use aoc::AoCInput;

fn solve_task(input: &str) -> (u64, u64) {
    let positions: Vec<u64> = input
        .trim()
        .split(',')
        .map(|p| p.parse().unwrap())
        .collect();

    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();

    let mut task1 = u64::MAX;
    for pos in min..=max {
        let fuel: u64 = positions.iter().map(|p| p.abs_diff(pos)).sum();
        if fuel < task1 {
            task1 = fuel;
        }
    }

    let mut task2 = u64::MAX;
    for pos in min..=max {
        let fuel: u64 = positions
            .iter()
            .map(|p| {
                // Calculate arithmetical sum of all numbers between p and pos
                let diff = p.abs_diff(pos);
                diff * (1 + diff) / 2
            })
            .sum();
        if fuel < task2 {
            task2 = fuel;
        }
    }

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2021, 7)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d07 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = "16,1,2,0,4,2,7,1,2,14";

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 37);
        assert_eq!(example2, 168);
    }
}
