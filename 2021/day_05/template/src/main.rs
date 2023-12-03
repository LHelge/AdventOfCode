fn solve_task(_input: &str) -> (u64, u64) {
    (0, 0)
}

fn main() {
    let input = aoc_input::get_input(
        XXXX,
        YY,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod yXXXXdYY {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#""#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 0);
        assert_eq!(example2, 0);
    }
}
