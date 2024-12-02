use aoc::AoCInput;

fn solve_task(input: &str) -> (i32, Option<usize>) {
    let mut task1 = 0;
    let mut task2 = None;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => task1 += 1,
            ')' => task1 -= 1,
            _ => panic!("Unknown character: {}", c),
        }

        if task1 < 0 && task2.is_none() {
            task2 = Some(i + 1);
        }
    }

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2015, 1)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2.unwrap());
}

#[cfg(test)]
mod y2015d01 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task("(())");
        assert_eq!(example1, 0);

        let (example1, _) = solve_task("()()");
        assert_eq!(example1, 0);

        let (example1, _) = solve_task("(((");
        assert_eq!(example1, 3);

        let (example1, _) = solve_task("(()(()(");
        assert_eq!(example1, 3);

        let (example1, _) = solve_task("))(((((");
        assert_eq!(example1, 3);

        let (example1, _) = solve_task("())");
        assert_eq!(example1, -1);

        let (example1, _) = solve_task("))(");
        assert_eq!(example1, -1);

        let (example1, _) = solve_task(")))");
        assert_eq!(example1, -3);

        let (example1, _) = solve_task(")())())");
        assert_eq!(example1, -3);

        let (_, example2) = solve_task(")");
        assert_eq!(example2, Some(1));

        let (_, example2) = solve_task("()())");
        assert_eq!(example2, Some(5));
    }
}
