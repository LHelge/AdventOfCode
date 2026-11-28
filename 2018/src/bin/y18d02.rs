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

struct Problem {
    boxes: Vec<Id>,
}

impl AoCProblem for Problem {
    type Solution1 = usize;
    type Solution2 = String;

    const YEAR: u16 = 2018;
    const DAY: u8 = 2;

    fn parse(input: &str) -> Result<Self> {
        Ok(Self {
            boxes: input.lines().map(|line| Id(line.into())).collect(),
        })
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
    Problem::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() -> Result<()> {
        let input = r#"abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab"#;

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part1()?, 12);

        let input = r#"abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"#;

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part2()?, String::from("fgij"));

        Ok(())
    }
}
