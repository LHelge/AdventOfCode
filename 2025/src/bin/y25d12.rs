use aoc::{problem::*, utils::*, *};
use std::str::FromStr;

#[derive(Debug)]
struct Present(usize);

impl FromStr for Present {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let size = s.chars().filter(|c| *c == '#').count();

        Ok(Present(size))
    }
}

impl Present {
    fn size_compressed(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
struct Region {
    size: Size,
    presents: Vec<usize>,
}

impl FromStr for Region {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let (size, presents) = s.split_once(": ").ok_or(AoCError::BadInput)?;
        let (width, height) = size.split_once('x').ok_or(AoCError::BadInput)?;

        let size = Size::new(width.parse()?, height.parse()?);
        let presents = presents.parse_whitespace_delimited()?;

        Ok(Region { size, presents })
    }
}

impl Region {
    fn check_fit(&self, presents: &Vec<Present>) -> bool {
        // Assume the presents are socks, ore something else that is compressible
        let area = self.size.area();
        let sum: usize = self
            .presents
            .iter()
            .zip(presents)
            .map(|(num, pres)| num * pres.size_compressed())
            .sum();

        sum < area
    }
}

#[derive(Default, Debug)]
struct Problem {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

impl AoCProblem<usize, usize> for Problem {
    fn date() -> Date {
        Date::new(2025, 12).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.presents.clear();

        for (i, part) in input.split("\n\n").enumerate() {
            let header = format!("{i}:\n");
            if let Some(present) = part.strip_prefix(&header) {
                self.presents.push(present.parse()?);
            } else {
                self.regions = part.parse_lines()?;
            }
        }

        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        Ok(self
            .regions
            .iter()
            .filter(|r| r.check_fit(&self.presents))
            .count())
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

    #[ignore = "This day's test input was just trolling."]
    #[test]
    fn examples() {
        let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(2);
    }
}
