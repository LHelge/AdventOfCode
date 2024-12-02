use aoc::AoCInput;

fn count_deeper(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.trim().parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|win| win[0] < win[1])
        .count()
}

fn count_deeper_filtered(input: &str) -> usize {
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

fn solve_task(input: &str) -> (usize, usize) {
    (count_deeper(input), count_deeper_filtered(input))
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2021, 1)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d01 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"199
        200
        208
        210
        200
        207
        240
        269
        260
        263"#;

        let (example1, example2) = solve_task(example_input);
        assert_eq!(example1, 7);
        assert_eq!(example2, 5);
    }
}
