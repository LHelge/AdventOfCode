use aoc::AoCInput;

fn solve_task(input: &str) -> (u32, u32) {
    let mut digits = String::from(input.trim());
    digits.push(input.chars().next().expect("Bad input"));

    let task1 = digits.as_bytes().windows(2).fold(0, |acc, w| {
        if w[0] == w[1] {
            acc + (w[0] - b'0') as u32
        } else {
            acc
        }
    });

    let digits = input.trim().as_bytes();
    let task2 = digits
        .iter()
        .zip(digits.iter().cycle().skip(digits.len() / 2))
        .fold(
            0,
            |acc, (a, b)| {
                if a == b {
                    acc + (a - b'0') as u32
                } else {
                    acc
                }
            },
        );

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2017, 1)
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
        // Task 1
        let (example1, _) = solve_task("1122");
        assert_eq!(example1, 3);
        let (example1, _) = solve_task("1111");
        assert_eq!(example1, 4);
        let (example1, _) = solve_task("1234");
        assert_eq!(example1, 0);
        let (example1, _) = solve_task("91212129");
        assert_eq!(example1, 9);

        // Task 2
        let (_, example2) = solve_task("1212");
        assert_eq!(example2, 6);
        let (_, example2) = solve_task("1221");
        assert_eq!(example2, 0);
        let (_, example2) = solve_task("123425");
        assert_eq!(example2, 4);
        let (_, example2) = solve_task("123123");
        assert_eq!(example2, 12);
        let (_, example2) = solve_task("12131415");
        assert_eq!(example2, 4);
    }
}
