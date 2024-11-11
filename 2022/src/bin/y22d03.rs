use aoc::AoCInput;

fn priority(item: char) -> Option<u64> {
    match item {
        'a'..='z' => Some(item as u64 - 'a' as u64 + 1),
        'A'..='Z' => Some(item as u64 - 'A' as u64 + 27),
        _ => None,
    }
}

fn solve_task(input: &str) -> (u64, u64) {
    let mut result1 = 0;
    let mut result2 = 0;

    let mut group: Vec<String> = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        // Part 1
        let count = line.len();
        let first_compartment = &line[..count / 2];
        let second_compartment = &line[count / 2..];
        for c in first_compartment.chars() {
            if second_compartment.contains(c) {
                result1 += priority(c).unwrap_or(0);
                break;
            }
        }

        // Part 2
        group.push(line.to_string());
        if group.len() == 3 {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    result2 += priority(c).unwrap_or(0);
                    break;
                }
            }
            group.clear();
        }
    }

    (result1, result2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2022, 3)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2022d03 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 157);
        assert_eq!(example2, 70);
    }
}
