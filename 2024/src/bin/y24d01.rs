use aoc::AoCInput;

fn solve_task(input: &str) -> (u32, u32) {
    let (task1, task2) = input.split_once(' ').unwrap();
    (task1.parse().unwrap(), task2.parse().unwrap())
}

fn main() {
    let input = AoCInput::from_env()
    .get_input(2024, 1)
    .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2024d01 {
    use super::*;

    #[test]
    fn examples() {
        // Task 1
        let (example1, _) = solve_task("1 2");
        assert_eq!(example1, 1);

        // Task 2
        let (_, example2) = solve_task("1 2");
        assert_eq!(example2, 2);
    }
}
