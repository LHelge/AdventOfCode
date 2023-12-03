use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Number {
    value: u64,
    area: (RangeInclusive<i64>, RangeInclusive<i64>),
}

/// Reads all part numbers and their areas from the input. Area is 1 unit outside of the actual number
fn read_schematic(input: &str) -> Vec<Number> {
    let mut numbers = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut line = line.trim().chars().enumerate();

        while let Some((x, c)) = line.next() {
            if c.is_numeric() {
                let mut digits = String::new();
                digits.push(c);

                for (_, c) in line.by_ref() {
                    if c.is_numeric() {
                        digits.push(c);
                    } else {
                        break;
                    }
                }

                let (x_start, x_end) = (x as i64 - 1, (x + digits.len()) as i64);
                let (y_start, y_end) = (y as i64 - 1, y as i64 + 1);

                numbers.push(Number {
                    value: digits.parse().unwrap(),
                    area: (x_start..=x_end, y_start..=y_end),
                });
            }
        }
    }

    numbers
}

fn solve_task(input: &str) -> (u64, u64) {
    let numbers = read_schematic(input);

    // Find all numbers that are adjacet to non-numberic characters
    let mut part_numbers: HashSet<Number> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                for n in numbers
                    .iter()
                    .filter(|n| n.area.0.contains(&(x as i64)) && n.area.1.contains(&(y as i64)))
                {
                    part_numbers.insert(n.clone());
                }
            }
        }
    }
    let task1 = part_numbers.iter().map(|n| n.value).sum::<u64>();

    // Find all instances of exactly two numbers that are adjacent to a '*'
    let mut gear_ratios: HashSet<(Number, Number)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            if c == '*' {
                let gear = numbers
                    .iter()
                    .filter(|n| n.area.0.contains(&(x as i64)) && n.area.1.contains(&(y as i64)))
                    .collect::<Vec<&Number>>();

                if gear.len() == 2 {
                    gear_ratios.insert((gear[0].clone(), gear[1].clone()));
                }
            }
        }
    }
    let task2 = gear_ratios
        .iter()
        .map(|(n1, n2)| n1.value * n2.value)
        .sum::<u64>();

    (task1, task2)
}

fn main() {
    let input = aoc_input::get_input(
        2023,
        3,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d03 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."#;

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 4361);
        assert_eq!(example2, 467835);
    }
}
