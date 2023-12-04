use std::collections::BTreeSet;

fn solve_task(input: &str) -> (i32, i32) {
    let calibrations: Vec<i32> = input
        .trim()
        .lines()
        .map(|f| {
            let (sign, num) = f.split_at(1);
            let num = num.parse::<u32>().unwrap() as i32;
            match sign {
                "+" => num,
                "-" => -num,
                _ => panic!("Bad input"),
            }
        })
        .collect();

    let task1 = calibrations.iter().sum();

    let mut task2 = 0;
    let mut visited: BTreeSet<i32> = BTreeSet::new();
    visited.insert(0);
    for cal in calibrations.iter().cycle() {
        task2 += cal;

        // Bail if there's no end in sight
        if !visited.insert(task2) || task2.abs() > 100000 {
            break;
        }
    }

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2018,
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
        // Task 1
        let (example1, _) = solve_task("+1\n-2\n+3\n+1");
        assert_eq!(example1, 3);
        let (example1, _) = solve_task("+1\n+1\n+1");
        assert_eq!(example1, 3);
        let (example1, _) = solve_task("+1\n+1\n-2");
        assert_eq!(example1, 0);
        let (example1, _) = solve_task("-1\n-2\n-3");
        assert_eq!(example1, -6);

        // Task 2
        let (_, example2) = solve_task("+1\n-1");
        assert_eq!(example2, 0);
        let (_, example2) = solve_task("+3\n+3\n+4\n-2\n-4");
        assert_eq!(example2, 10);
        let (_, example2) = solve_task("-6\n+3\n+8\n+5\n-6");
        assert_eq!(example2, 5);
        let (_, example2) = solve_task("+7\n+7\n-2\n-7\n-4");
        assert_eq!(example2, 14);
    }
}
