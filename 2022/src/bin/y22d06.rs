use std::collections::HashSet;

use aoc::AoCInput;

fn has_double(signal: &str) -> bool {
    let mut chars = HashSet::new();
    for c in signal.chars() {
        if !chars.insert(c) {
            return true;
        }
    }
    false
}

fn find_marker(input: &str, len: usize) -> Option<u64> {
    for i in len..input.len() {
        if !has_double(&input[i - len..i]) {
            return Some(i as u64);
        }
    }
    None
}

fn solve_task(input: &str) -> (u64, u64) {
    (
        find_marker(input, 4).expect("Bad input"),
        find_marker(input, 14).expect("Bad input"),
    )
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2022, 6)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d06 {
    use super::*;

    #[test]
    fn examples() {
        let (task1, task2) = solve_task("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(task1, 7);
        assert_eq!(task2, 19);

        let (task1, task2) = solve_task("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(task1, 5);
        assert_eq!(task2, 23);

        let (task1, task2) = solve_task("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(task1, 6);
        assert_eq!(task2, 23);

        let (task1, task2) = solve_task("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(task1, 10);
        assert_eq!(task2, 29);

        let (task1, task2) = solve_task("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(task1, 11);
        assert_eq!(task2, 26);
    }
}
