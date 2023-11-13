use std::collections::BTreeSet;

pub fn count_calories(input: &str) -> BTreeSet<u64> {
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

    elves
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_2022_01() {
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

        let result = *count_calories(input).last().unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn tasks_2022_01() {
        let input = aoc_input::get_input(
            2022,
            1,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        let elves = count_calories(&input);

        // Task 1
        let task1 = *elves.last().unwrap();
        assert_eq!(task1, 71934);

        // Task 2
        let task2: u64 = elves.iter().rev().take(3).sum();
        assert_eq!(task2, 211447);
    }
}
