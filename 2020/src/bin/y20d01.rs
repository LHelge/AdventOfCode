use std::collections::BTreeSet;

fn solve_task(input: &str) -> (u32, u32) {
    let expenses = input
        .lines()
        .map(|l| l.trim().parse::<u32>().expect("Bad input"))
        .collect::<BTreeSet<u32>>();

    let mut task1 = 0;
    for expense in &expenses {
        let target = 2020 - expense;
        if expenses.contains(&target) {
            task1 = expense * target;
            break;
        }
    }

    let mut task2 = None;
    for expense1 in &expenses {
        for expense2 in &expenses {
            if expense1 == expense2 || expense1 + expense2 > 2020 {
                continue;
            }

            let target = 2020 - expense1 - expense2;
            if expenses.contains(&target) {
                task2 = Some(expense1 * expense2 * target);
                break;
            }
        }

        if task2.is_some() {
            break;
        }
    }

    (task1, task2.unwrap())
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
        let input = r#"1721
979
366
299
675
1456"#;
        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 514579);
        assert_eq!(example2, 241861950);
    }
}
