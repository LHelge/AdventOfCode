use aoc::AoCInput;

trait MatchstickString {
    fn unescaped_len(&self) -> usize;
    fn escaped_len(&self) -> usize;
}

impl MatchstickString for &str {
    fn unescaped_len(&self) -> usize {
        let mut chars = self.chars();
        let mut len = 0;

        while let Some(c) = chars.next() {
            match c {
                '\\' => match chars.next().unwrap() {
                    '\\' => len += 1,
                    '"' => len += 1,
                    'x' => {
                        chars.next();
                        chars.next();
                        len += 1;
                    }
                    _ => panic!("Invalid escape sequence"),
                },
                _ => len += 1,
            }
        }

        len - 2 // remove the surrounding quotes
    }

    fn escaped_len(&self) -> usize {
        let mut len = 0;

        for c in self.chars() {
            match c {
                '\\' => len += 2,
                '"' => len += 2,
                _ => len += 1,
            }
        }

        len + 2 // add the surrounding quotes
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let task1 = input.lines().map(|l| l.len() - l.unescaped_len()).sum();

    let task2 = input.lines().map(|l| l.escaped_len() - l.len()).sum();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2015, 8)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d08 {
    use super::*;

    #[test]
    fn examples() {
        let example = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

        let (example1, example2) = solve_task(example);
        assert_eq!(example1, 12);
        assert_eq!(example2, 19);
    }

    #[test]
    fn unescaped() {
        assert_eq!(r#""""#.unescaped_len(), 0);
        assert_eq!(r#""abc""#.unescaped_len(), 3);
        assert_eq!(r#""aaa\"aaa""#.unescaped_len(), 7);
        assert_eq!(r#""\x27""#.unescaped_len(), 1);
    }

    #[test]
    fn escaped() {
        assert_eq!(r#""""#.escaped_len(), 6);
        assert_eq!(r#""abc""#.escaped_len(), 9);
        assert_eq!(r#""aaa\"aaa""#.escaped_len(), 16);
        assert_eq!(r#""\x27""#.escaped_len(), 11);
    }
}
