use aoc::AoCError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum SpringState {
    Unknown,
    Operational,
    Damaged,
}

impl From<char> for SpringState {
    fn from(c: char) -> Self {
        match c {
            '?' => SpringState::Unknown,
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            _ => panic!("Invalid state character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Spring {
    states: Vec<SpringState>,
    counts: Vec<usize>,
}

impl FromStr for Spring {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states, counts) = s.split_once(' ').ok_or(AoCError::BadInput)?;

        let states = states.chars().map(SpringState::from).collect();
        let counts = counts
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Ok(Self { states, counts })
    }
}

impl Spring {
    fn is_valid(&self) -> bool {
        let mut sizes: Vec<usize> = Vec::new();
        let mut size: Option<usize> = None;

        for s in self.states.iter() {
            match (s, size) {
                (SpringState::Unknown, _) => return false,
                (SpringState::Damaged, None) => size = Some(1),
                (SpringState::Damaged, Some(s)) => size = Some(s + 1),
                (SpringState::Operational, Some(s)) => {
                    sizes.push(s);
                    size = None;
                }
                _ => (),
            }
        }
        if let Some(size) = size {
            sizes.push(size);
        }

        sizes.eq(&self.counts)
    }

    fn unfold(&mut self) {
        let states = self.states.clone();
        let sizes = self.counts.clone();

        for _ in 0..4 {
            self.states.push(SpringState::Unknown);
            self.states.extend(states.iter().cloned());
            self.counts.extend(sizes.iter().cloned());
        }
    }
}

fn count_correct_arrangements(spring: &Spring) -> usize {
    // Find first index with unknown state
    if let Some(idx) = spring
        .states
        .iter()
        .position(|s| *s == SpringState::Unknown)
    {
        // Create one version with operational and one with damaged
        let mut spring1 = spring.clone();
        spring1.states[idx] = SpringState::Operational;

        let mut spring2 = spring.clone();
        spring2.states[idx] = SpringState::Damaged;

        count_correct_arrangements(&spring1) + count_correct_arrangements(&spring2)
    } else if spring.is_valid() {
        1
    } else {
        0
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let mut springs: Vec<Spring> = input.lines().map(|l| l.parse().unwrap()).collect();

    let task1 = springs.iter().map(count_correct_arrangements).sum();

    springs.iter_mut().for_each(|s| s.unfold());

    let task2 = 0;
    //let task2 = springs
    //    .iter()
    //    .map(|s| count_correct_arrangements(s, &mut HashMap::new()))
    //    .sum();

    (task1, task2)
}

fn main() {
    let input = aoc::get_input(
        2023,
        12,
        &std::env::var("SESSION").expect("SESSION environment variable not set"),
    )
    .unwrap();

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2023d12 {
    use super::*;

    #[test]
    fn examples() {
        let example_input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

        let (example1, _example2) = solve_task(example_input);

        assert_eq!(example1, 21);
        //assert_eq!(example2, 525152);
    }
}
