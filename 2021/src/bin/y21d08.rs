use std::collections::{HashMap, HashSet};

const UNIQUE_LENGTH: [usize; 4] = [2, 4, 3, 7];

///  0000
/// 1    2
/// 1    2
///  3333
/// 4    5
/// 4    5
///  6666
struct Wiring {
    unclear: Vec<HashSet<char>>,
    clear: Option<HashMap<char, u8>>,
}

impl Wiring {
    fn new() -> Self {
        let mut all = HashSet::new();
        for c in 'a'..='g' {
            all.insert(c);
        }

        Self {
            unclear: vec![all.clone(); 7],
            clear: None,
        }
    }

    fn process(&mut self, sequence: &str) {
        match sequence.len() {
            2 => {
                // Must be 1
                // Only retain numbers from sequence in 2 & 5
                self.unclear[2].retain(|&c| sequence.contains(c));
                self.unclear[5].retain(|&c| sequence.contains(c));

                // Remove from all other segments
                self.unclear[0].retain(|&c| !sequence.contains(c));
                self.unclear[1].retain(|&c| !sequence.contains(c));
                self.unclear[3].retain(|&c| !sequence.contains(c));
                self.unclear[4].retain(|&c| !sequence.contains(c));
                self.unclear[6].retain(|&c| !sequence.contains(c));
            }
            3 => {
                // Must be 7
                // only retain numbers from sequence in 0, 2 & 5
                self.unclear[0].retain(|&c| sequence.contains(c));
                self.unclear[2].retain(|&c| sequence.contains(c));
                self.unclear[5].retain(|&c| sequence.contains(c));

                // Remove from all other segments
                self.unclear[1].retain(|&c| !sequence.contains(c));
                self.unclear[3].retain(|&c| !sequence.contains(c));
                self.unclear[4].retain(|&c| !sequence.contains(c));
                self.unclear[6].retain(|&c| !sequence.contains(c));
            }
            4 => {
                // Must be 4, only retain numbers from sequence in 1, 2, 3 & 5
                self.unclear[1].retain(|&c| sequence.contains(c));
                self.unclear[2].retain(|&c| sequence.contains(c));
                self.unclear[3].retain(|&c| sequence.contains(c));
                self.unclear[5].retain(|&c| sequence.contains(c));

                // Remove from all other segments
                self.unclear[0].retain(|&c| !sequence.contains(c));
                self.unclear[4].retain(|&c| !sequence.contains(c));
                self.unclear[6].retain(|&c| !sequence.contains(c));
            }
            5 => {} // Does not add meaningful information since 2 and 5 activate all segments
            6 => {
                // Must be 0, 6 or 9
                self.unclear[0].retain(|&c| sequence.contains(c));
                self.unclear[1].retain(|&c| sequence.contains(c));
                self.unclear[5].retain(|&c| sequence.contains(c));
                self.unclear[6].retain(|&c| sequence.contains(c));
            }
            7 => {} // Does not add meaningful information since all segments are active
            _ => panic!("Unexpected sequence length: {}", sequence.len()),
        }
    }

    fn clear(&mut self) -> Result<(), ()> {
        // Check if there are any segments with only one possible value and remove from others
        let mut found: Vec<char> = vec![];
        for v in &self.unclear {
            if v.len() == 1 {
                found.push(*v.iter().next().unwrap());
            }
        }

        for v in self.unclear.iter_mut() {
            if v.len() > 1 {
                v.retain(|c| !found.contains(c));
            }
        }

        dbg!(&self.unclear);

        if self.unclear.iter().any(|u| u.len() != 1) {
            return Err(());
        }

        let mut clear = HashMap::new();
        for (i, v) in self.unclear.iter().enumerate() {
            clear.insert(*v.iter().next().unwrap(), i as u8);
        }

        Ok(())
    }

    fn value(&self, sequence: &str) -> Result<u8, ()> {
        todo!();
    }
}

fn solve_task(input: &str) -> (u64, u64) {
    let patterns = input
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .collect::<Vec<_>>();

    let mut task1 = 0;
    for (_, output) in &patterns {
        task1 += output
            .split_whitespace()
            .filter(|o| UNIQUE_LENGTH.contains(&o.len()))
            .count() as u64;
    }

    let mut task2 = 0;
    for (input, output) in &patterns {
        let mut wiring = Wiring::new();
        for sequence in input.split_whitespace() {
            wiring.process(sequence);
        }
        wiring.clear();
    }

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2021,
        8,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2021d08 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

        let (example1, _example2) = solve_task(example_input);

        //assert_eq!(example1, 26);
        //assert_eq!(example2, 0);
    }
}
