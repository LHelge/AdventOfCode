pub fn count_deeper(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|win| win[0] < win[1])
        .count()
}

pub fn count_deeper_filtered(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|win| win.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|win| win[0] < win[1])
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01_example() {
        let input = r#"199
        200
        208
        210
        200
        207
        240
        269
        260
        263"#;

        let result = count_deeper(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn day_01_tasks() {
        let input = aoc_input::get_input(
            2021,
            1,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        // Task 1
        let task1 = count_deeper(&input);
        assert_eq!(task1, 1791);

        // Task 2
        let task2 = count_deeper_filtered(&input);
        assert_eq!(task2, 1822);
    }
}
