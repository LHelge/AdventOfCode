use aoc::AoCInput;

fn fuel_for_fuel(fuel: u32) -> u32 {
    let mut fuel = fuel / 3;

    if fuel <= 2 {
        0
    } else {
        fuel -= 2;
        fuel + fuel_for_fuel(fuel)
    }
}

fn solve_task(input: &str) -> (u32, u32) {
    let task1 = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap() / 3 - 2)
        .sum::<u32>();

    let task2 = input
        .lines()
        .map(|f| fuel_for_fuel(f.parse().expect("Bad input")))
        .sum::<u32>();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2019,1)
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
        let (example1, _) = solve_task("12");
        assert_eq!(example1, 2);

        let (example1, example2) = solve_task("14");
        assert_eq!(example1, 2);
        assert_eq!(example2, 2);

        let (example1, example2) = solve_task("1969");
        assert_eq!(example1, 654);
        assert_eq!(example2, 966);

        let (example1, example2) = solve_task("100756");
        assert_eq!(example1, 33583);
        assert_eq!(example2, 50346);
    }
}
