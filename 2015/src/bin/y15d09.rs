use aoc::Permutable;
use std::collections::{HashMap, HashSet};

fn solve_task(input: &str) -> (u64, u64) {
    let mut distances: HashMap<(&str, &str), u64> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let from = parts.next().unwrap();
        let to = parts.nth(1).unwrap();
        let distance = parts.nth(1).unwrap().parse::<u64>().unwrap();

        distances.insert((from, to), distance);
        distances.insert((to, from), distance);
        cities.insert(from);
        cities.insert(to);
    }

    let cities = cities.into_iter().collect::<Vec<_>>();

    let mut task1 = u64::MAX;
    let mut task2 = u64::MIN;
    //let mut task2 = u64::MIN;
    for permutation in cities.permute() {
        let distance = permutation
            .windows(2)
            .map(|w| distances.get(&(w[0], w[1])).unwrap())
            .sum();

        task1 = task1.min(distance);
        task2 = task2.max(distance);
    }

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2015,
        9,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d09 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

        let (example1, _example2) = solve_task(example_input);

        assert_eq!(example1, 605);
        //assert_eq!(example2, 0);
    }
}
