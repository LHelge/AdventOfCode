use aoc::AoCInput;


fn solve_task(_input: &str) -> (usize, usize) {
    (0, 0)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2020, 5)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d05 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task("BFFFBBFRRR");
        assert_eq!(example1, 567);

        let (example2, _) = solve_task("FFFBBBFRRR");
        assert_eq!(example2, 119);

        let (example3, _) = solve_task("BBFFBBFRLL");
        assert_eq!(example3, 820);
    }
}
