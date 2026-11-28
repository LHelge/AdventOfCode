use aoc::{problem::*, utils::*, *};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Id(String);

impl Id {
    fn has_count(&self, count: usize) -> bool {
        self.0.chars().counts().values().any(|&c| c == count)
    }

    fn match_but_one(&self, other: &Self) -> Option<String> {
        let common: String = self
            .0
            .chars()
            .zip(other.0.chars())
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .collect();

        if self.0.len() - 1 == common.len() {
            Some(common)
        } else {
            None
        }
    }
}

#[derive(Default)]
struct Problem {
    boxes: Vec<Id>,
}

impl AoCProblem<usize, String> for Problem {
    fn date() -> Date {
        Date::new(2018, 2).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.boxes.clear();

        for line in input.lines() {
            self.boxes.push(Id(line.into()));
        }

        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        let twos = self.boxes.iter().filter(|id| id.has_count(2)).count();
        let threes = self.boxes.iter().filter(|id| id.has_count(3)).count();

        Ok(twos * threes)
    }

    fn part2(&self) -> Result<String> {
        self.boxes
            .pairs()
            .find_map(|(id1, id2)| id1.match_but_one(id2))
            .ok_or(AoCError::BadInput)
    }
}

fn main() -> Result<()> {
    let mut problem = Problem::default();
    let solution = problem.solve()?;

    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(12);

        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;

        problem.parse(input).unwrap();
        problem.test_part2(String::from("fgij"));
    }
}
