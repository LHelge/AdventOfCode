fn solve_task(input: &str) -> (usize, usize) {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .trim_start_matches("Time:")
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distance = lines
        .next()
        .unwrap()
        .trim_start_matches("Distance:")
        .split_whitespace()
        .map(|t| t.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut task1 = 1;
    for (&t, &d) in time.iter().zip(distance.iter()) {
        let wins = (0..t).filter(|push| (t - push) * push > d).count();
        task1 *= wins;
    }

    // Task 2
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .trim()
        .trim_start_matches("Time:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .trim()
        .trim_start_matches("Distance:")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let task2 = (0..time)
        .filter(|push| (time - push) * push > distance)
        .count();

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2023,
        6,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d06 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 288);
        assert_eq!(example2, 71503);
    }
}
