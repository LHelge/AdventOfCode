fn solve_task(_input: &str) -> (u32, u32) {
    (0, 0)
}

fn main() {
    let input = aoc::get_input(
        2020,
        1,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d01 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task("1122");
        assert_eq!(example1, 0);
    }
}
