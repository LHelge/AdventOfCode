use std::fmt::Display;

struct PasswordIterator {
    password: String,
}

impl Iterator for PasswordIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut password = self.password.clone().into_bytes();

        let mut i = password.len() - 1;
        while i > 0 && password[i] == b'z' {
            password[i] = b'a';
            i -= 1;
        }

        password[i] += 1;

        self.password = String::from_utf8(password).unwrap();

        Some(self.password.clone())
    }
}

trait PasswordString
where
    Self: Display,
{
    fn is_valid(&self) -> bool {
        let password = self.to_string().into_bytes();

        // must be 8 characters
        if password.len() != 8 {
            return false;
        }

        // must be lowercase
        if password.iter().any(|c| !c.is_ascii_lowercase()) {
            return false;
        }

        // i, o and l is not allowed
        if password.contains(&b'i') || password.contains(&b'o') || password.contains(&b'l') {
            return false;
        }

        let mut has_triplet = false;
        let mut pairs = 0;
        for (i, w) in password.windows(3).enumerate() {
            if w[0] + 1 == w[1] && w[1] + 1 == w[2] {
                has_triplet = true;
            }

            if (w[0] != w[1] && w[1] == w[2]) || (i == 0 && w[0] == w[1] && w[1] != w[2]) {
                pairs += 1;
            }
        }

        has_triplet && pairs >= 2
    }

    fn pass_iter(&self) -> PasswordIterator {
        PasswordIterator {
            password: self.to_string(),
        }
    }
}

impl PasswordString for String {}

fn solve_task(input: &str) -> (String, String) {
    let password = input.to_string();
    let task1 = password.pass_iter().find(|p| p.is_valid()).unwrap();
    let task2 = task1.pass_iter().find(|p| p.is_valid()).unwrap();
    (task1, task2)
}

fn main() {
    let (task1, task2) = solve_task("cqjxjnds");

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d11 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _example2) = solve_task("abcdefgh");
        assert_eq!(example1, "abcdffaa");
        //assert_eq!(example2, 0);

        let (example1, _example2) = solve_task("ghijklmn");
        assert_eq!(example1, "ghjaabcc");
        //assert_eq!(example2, 0);
    }

    #[test]
    fn validation() {
        assert!(!String::from("hijklmmn").is_valid());
        assert!(!String::from("abbceffg").is_valid());
        assert!(!String::from("abbcegjk").is_valid());
        assert!(String::from("abcdffaa").is_valid());
        assert!(String::from("ghjaabcc").is_valid());
    }
}
