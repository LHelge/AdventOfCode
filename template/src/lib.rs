pub fn solve_task(_input: &str) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_XXXX_YY() {
        let input = r#""#;

        let (example1, example2) = solve_task(input);

        assert_eq!(example1, 0);
        assert_eq!(example2, 0);
    }

    #[test]
    fn tasks_XXXX_YY() {
        let input = aoc_input::get_input(
            XXXX,
            YY,
            &std::env::var("SESSION").expect("SESSION environment variable not set"),
        )
        .unwrap();

        let (task1, task2) = solve_task(&input);

        assert_eq!(task1, 0);
        assert_eq!(task2, 0);
    }
}
