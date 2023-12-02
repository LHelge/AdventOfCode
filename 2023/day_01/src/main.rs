fn first_string_digit(line: &str) -> Option<(usize, char)> {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut found: Option<(usize, char)> = None;
    for (i, number) in numbers.iter().enumerate() {
        if let Some(pos) = line.find(number) {
            if found.is_none() || pos < found.unwrap().0 {
                found = Some((pos, ('0' as u8 + i as u8) as char));
            }
        }
    }

    found
}

fn first_digit(line: &str) -> Option<(usize, char)> {
    line.chars().enumerate().find(|(_, c)| c.is_numeric())
}

fn last_string_digit(line: &str) -> Option<(usize, char)> {
    let numbers = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut found: Option<(usize, char)> = None;
    for (i, number) in numbers.iter().enumerate() {
        if let Some(pos) = line.rfind(number) {
            if found.is_none() || pos > found.unwrap().0 {
                found = Some((pos, ('0' as u8 + i as u8) as char));
            }
        }
    }

    found
}

fn last_digit(line: &str) -> Option<(usize, char)> {
    if let Some((pos, digit)) = line.chars().rev().enumerate().find(|(_, c)| c.is_numeric()) {
        return Some((line.len() - pos - 1, digit));
    }
    None
}

fn solve_task(input: &str) -> (u64, u64) {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input.lines() {
        let line = line.trim();

        let first_digit = first_digit(line);
        let last_digit = last_digit(line);

        let mut number1 = String::new();
        number1.push(first_digit.unwrap_or((0, '0')).1);
        number1.push(last_digit.unwrap_or((0, '0')).1);
        sum1 += number1.parse::<u64>().unwrap();

        let first_string_digit = first_string_digit(line);
        let last_string_digit = last_string_digit(line);

        let mut number2 = String::new();
        match (first_digit, first_string_digit) {
            (Some((p1, d1)), Some((p2, d2))) => {
                if p2 <= p1 {
                    number2.push(d2);
                } else {
                    number2.push(d1);
                }
            }
            (Some((_, d)), None) => number2.push(d),
            (None, Some((_, d))) => number2.push(d),
            (None, None) => panic!("No digit found"),
        }
        match (last_digit, last_string_digit) {
            (Some((p1, d1)), Some((p2, d2))) => {
                if p2 >= p1 {
                    number2.push(d2);
                } else {
                    number2.push(d1);
                }
            }
            (Some((_, d)), None) => number2.push(d),
            (None, Some((_, d))) => number2.push(d),
            (None, None) => panic!("No digit found"),
        }
        sum2 += number2.parse::<u64>().unwrap();
    }

    (sum1, sum2)
}

fn main() {
    let input = aoc_input::get_input(
        2023,
        1,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d01 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"#;

        let (example1, _) = solve_task(example_input);

        assert_eq!(example1, 142);

        let example_input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;

        let (_, example2) = solve_task(example_input);
        assert_eq!(example2, 281);
    }
}
