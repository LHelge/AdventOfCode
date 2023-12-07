trait Unescape {
    fn unescaped_len(&self) -> usize;
}

impl Unescape for &str {
    fn unescaped_len(&self) -> usize {
        println!("\n{} : {}", self, self.len());
        let s = String::from(&self[1..(self.len() - 1)]);

        let s = s.replace("\\\\", "\\");
        let mut s = s.replace("\\\"", "\"");

        let hex = s
            .chars()
            .collect::<Vec<char>>()
            .windows(4)
            .enumerate()
            .filter_map(|(i, w)| {
                match (
                    w[0],
                    w[1],
                    u8::from_str_radix(&String::from_iter([w[2], w[3]]), 16),
                ) {
                    ('\\', 'x', Ok(c)) => Some((i, c as char)),
                    _ => None,
                }
            })
            .collect::<Vec<(usize, char)>>();

        for (i, c) in hex.iter().rev() {
            // Remove the hex escape
            s.remove(*i);
            s.remove(*i);
            s.remove(*i);
            s.remove(*i);
            s.insert(*i, *c);
        }

        println!("{} : {}", s, s.len());

        s.len()
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let task1 = input
        .trim()
        .lines()
        .map(|l| l.len() - l.unescaped_len())
        .sum();

    (task1, 0)
}

fn main() {
    let input = aoc::get_input(
        2015,
        8,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

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

        let (example1, _example2) = solve_task(example);
        assert_eq!(example1, 12);
    }
}
