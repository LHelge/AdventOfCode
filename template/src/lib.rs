pub fn solve_task(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_XXXX_YY() {
        let input = r#""#;

        let result = solve_task(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn tasks_XXXX_YY() {
        let input = aoc_input::get_input(
            XXXX,
            YY,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        let result = solve_task(&input);

        // Task 1
        let task1 = result.clone();
        assert_eq!(task1, 0);

        // Task 2
        let task2 = result.clone();
        assert_eq!(task2, 0);
    }
}
