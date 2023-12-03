fn simulate_fish(initial: &[u64; 9], days: usize) -> u64 {
    let mut current = *initial;
    for _ in 0..days {
        let spawned = current[0];

        current[0] = current[1];
        current[1] = current[2];
        current[2] = current[3];
        current[3] = current[4];
        current[4] = current[5];
        current[5] = current[6];
        current[6] = current[7] + spawned;
        current[7] = current[8];
        current[8] = spawned;
    }

    current.iter().sum()
}

fn solve_task(input: &str) -> (u64, u64) {
    let mut initial = [0; 9];

    for fish in input.split(',') {
        let fish = fish.trim().parse::<usize>().unwrap();
        initial[fish] += 1;
    }

    let task1 = simulate_fish(&initial, 80);
    let task2 = simulate_fish(&initial, 256);

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2021,
        6,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d06 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = "3,4,3,1,2";

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 5934);
        assert_eq!(example2, 26984457539);
    }
}
