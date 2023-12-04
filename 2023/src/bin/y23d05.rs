use aoc::AoCError;

fn solve_task(_input: &str) -> Result<(u64, u64), AoCError> {
    Ok((0, 0))
}

fn main() {
    let input = aoc::get_input(
        2023,
        4,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input).expect("Error while solving task");

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d04 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#""#;

        let (example1, _example2) = solve_task(example_input).expect("Error while solving task");

        assert_eq!(example1, 0);
        //assert_eq!(example2, 30);
    }
}
