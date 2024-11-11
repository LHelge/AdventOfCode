use aoc::AoCInput;

fn extrapolate(numbers: &mut Vec<i64>) {
    if numbers.iter().all(|n| *n == 0) {
        numbers.push(0);
        numbers.push(0);
    } else {
        let mut new_numbers: Vec<_> = numbers.windows(2).map(|w| w[1] - w[0]).collect();
        extrapolate(&mut new_numbers);
        numbers.push(numbers.last().unwrap() + new_numbers.last().unwrap());
        numbers.insert(0, numbers.first().unwrap() - new_numbers.first().unwrap());
    }
}

fn solve_task(input: &str) -> (i64, i64) {
    let (task1, task2) = input
        .lines()
        .map(|l| {
            let mut numbers = l
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect();
            extrapolate(&mut numbers);
            (*numbers.last().unwrap(), *numbers.first().unwrap())
        })
        .fold((0, 0), |(t1, t2), (n1, n2)| (t1 + n1, t2 + n2));

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2023, 9)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d09 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 114);
        assert_eq!(example2, 2);
    }
}
