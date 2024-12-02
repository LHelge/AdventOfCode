use std::collections::HashSet;

use aoc::AoCInput;

fn solve_task(input: &str) -> (usize, usize) {
    let task1 = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|l| l.chars())
                .fold(HashSet::new(), |mut set, c| {
                    set.insert(c);
                    set
                })
                .len()
        })
        .sum();

    let task2 = input
        .split("\n\n")
        .map(|group| {
            let mut people = group.lines();
            let mut answers: HashSet<char> = HashSet::from_iter(people.next().unwrap().chars());
            for p in people {
                answers.retain(|c| p.contains(*c));
            }
            answers.len()
        })
        .sum();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2020, 6)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2020d06 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 11);
        assert_eq!(example2, 6);
    }
}
