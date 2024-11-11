use aoc::AoCInput;

trait NiceString {
    fn is_nice(&self) -> bool;
    fn is_nicer(&self) -> bool;
}

impl NiceString for &str {
    fn is_nice(&self) -> bool {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

        let wovels = self.chars().filter(|c| VOWELS.contains(c)).count();
        let has_double = self.chars().zip(self.chars().skip(1)).any(|(a, b)| a == b);
        let has_bad = self.contains("ab")
            || self.contains("cd")
            || self.contains("pq")
            || self.contains("xy");

        wovels >= 3 && has_double && !has_bad
    }

    fn is_nicer(&self) -> bool {
        let chars = self.chars().collect::<Vec<char>>();

        let has_pair = chars.windows(2).enumerate().any(|(i, w)| {
            chars
                .windows(2)
                .enumerate()
                .any(|(j, w2)| i.abs_diff(j) > 1 && w == w2)
        });
        let has_repeat = chars.windows(3).any(|w| w[0] == w[2]);

        has_pair && has_repeat
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let task1 = input.trim().lines().filter(|l| l.is_nice()).count();
    let task2 = input.trim().lines().filter(|l| l.is_nicer()).count();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2015, 5)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d05 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, _) = solve_task("ugknbfddgicrmopn");
        assert_eq!(example1, 1);

        let (example1, _) = solve_task("aaa");
        assert_eq!(example1, 1);

        let (example1, _) = solve_task("jchzalrnumimnmhp");
        assert_eq!(example1, 0);

        let (example1, _) = solve_task("haegwjzuvuyypxyu");
        assert_eq!(example1, 0);

        let (example1, _) = solve_task("dvszwmarrgswjxmb");
        assert_eq!(example1, 0);

        let (_, example2) = solve_task("qjhvhtzxzqqjkmpb");
        assert_eq!(example2, 1);

        let (_, example2) = solve_task("xxyxx");
        assert_eq!(example2, 1);

        let (_, example2) = solve_task("uurcxstgmygtbstg");
        assert_eq!(example2, 0);

        let (_, example2) = solve_task("ieodomkazucvgmuy");
        assert_eq!(example2, 0);
    }
}
