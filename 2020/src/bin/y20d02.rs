use aoc::AoCInput;

#[derive(Debug)]
struct Rule {
    num: (usize, usize),
    character: char,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (num, c) = value.split_once(' ').unwrap();
        let num = num.split_once('-').unwrap();

        Self {
            num: (num.0.parse().unwrap(), num.1.parse().unwrap()),
            character: c.chars().next().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Password<'a> {
    rule: Rule,
    password: &'a str,
}

impl<'a> From<&'a str> for Password<'a> {
    fn from(value: &'a str) -> Self {
        let (rule, password) = value.split_once(':').unwrap();
        Self {
            rule: rule.into(),
            password: password.trim(),
        }
    }
}

impl Password<'_> {
    fn is_valid1(&self) -> bool {
        let num = self
            .password
            .chars()
            .filter(|&c| c == self.rule.character)
            .count();
        num >= self.rule.num.0 && num <= self.rule.num.1
    }

    fn is_valid2(&self) -> bool {
        let cond1 = self
            .password
            .chars()
            .nth(self.rule.num.0 - 1)
            .is_some_and(|c| c == self.rule.character);
        let cond2 = self
            .password
            .chars()
            .nth(self.rule.num.1 - 1)
            .is_some_and(|c| c == self.rule.character);

        cond1 ^ cond2
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let passwords: Vec<Password> = input.lines().map(|line| line.into()).collect();

    let task1 = passwords.iter().filter(|&p| p.is_valid1()).count();
    let task2 = passwords.iter().filter(|&p| p.is_valid2()).count();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .expect("SESSION env variable not found")
        .get_input(2020, 2)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2020d02 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc"#;
        let (example1, example2) = solve_task(input);
        assert_eq!(example1, 2);
        assert_eq!(example2, 1);
    }
}
