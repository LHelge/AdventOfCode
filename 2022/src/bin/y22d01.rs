use std::collections::BTreeSet;

use aoc::AoCInput;

fn count_calories(input: &str) -> BTreeSet<u64> {
    let mut elves = BTreeSet::new();

    let mut total = 0;
    for line in input.lines() {
        match line.trim().parse::<u64>() {
            Ok(n) => total += n,
            Err(_) => {
                elves.insert(total);
                total = 0;
            }
        }
    }
    elves.insert(total);

    elves
}

fn solve_task(input: &str) -> (u64, u64) {
    let elves = count_calories(input);

    (*elves.last().unwrap(), elves.iter().rev().take(3).sum())
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2022, 1)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d01 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000"#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 24000);
        assert_eq!(example2, 45000);
    }
}
