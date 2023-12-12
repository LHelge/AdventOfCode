use aoc::AoCError;
use memoize::memoize;
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

fn count_correct_arrangements(states: &[SpringState], counts: &[usize]) -> usize {
    // If there are no possibly damaged springs left, counts must also be empty
    let Some(first_state) = states.first() else {
        if counts.is_empty() {
            return 1;
        } else {
            return 0;
        }
    };

    // If counts is empty, there can not be any damaged springs left
    let Some(&first_count) = counts.first() else {
        if states.contains(&SpringState::Damaged) {
            return 0;
        } else {
            return 1;
        }
    };

    if counts.len() == 1 && states.len() == first_count {
        if !states.contains(&SpringState::Operational) {
            return 1;
        } else {
            return 0;
        }
    }

    let mut arrangements = 0;

    if first_state == &SpringState::Unknown || first_state == &SpringState::Operational {
        // assume unknown is operational and skip it
        arrangements += count_correct_arrangements(&states[1..], counts);
    }

    // Assume start of block (either unknown or damaged)
    if first_state == &SpringState::Unknown || first_state == &SpringState::Damaged {
        // assume unknown is damaged
        if first_count <= states.len()
            && !states[0..first_count].contains(&SpringState::Operational)
            && (first_count == states.len() || states[first_count] != SpringState::Damaged)
        {
            let skip = if first_count == states.len() {
                first_count
            } else {
                first_count + 1
            };
            arrangements += count_correct_arrangements(&states[skip..], &counts[1..]);
        }
    }

    arrangements
}

fn solve_task(input: &str) -> (usize, usize) {
    let mut springs: Vec<Spring> = input.lines().map(|l| l.parse().unwrap()).collect();

    let task1 = springs
        .iter()
        .map(|s| count_correct_arrangements(&s.states, &s.counts))
        .sum();

    //let task2 = 0;
    springs.iter_mut().for_each(|s| s.unfold());
    let task2 = springs
        .iter()
        .map(|s| count_correct_arrangements(&s.states, &s.counts))
        .inspect(|c| println!("arrangements: {}", c))
        .sum();

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

        let (example1, example2) = solve_task(example_input);

        assert_eq!(example1, 21);
        assert_eq!(example2, 525152);
    }
}
