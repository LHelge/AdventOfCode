use aoc::{problem::*, utils::*, *};
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
    hash::Hash,
    ops::Sub,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl FromStr for JunctionBox {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self> {
        let mut s = s.split(',');

        let x = s.next().ok_or(AoCError::BadInput)?.parse()?;
        let y = s.next().ok_or(AoCError::BadInput)?.parse()?;
        let z = s.next().ok_or(AoCError::BadInput)?.parse()?;

        Ok(Self::new(x, y, z))
    }
}

impl JunctionBox {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn distance_to_squared(&self, other: &JunctionBox) -> i64 {
        self.x.sub(other.x).pow(2) + self.y.sub(other.y).pow(2) + self.z.sub(other.z).pow(2)
    }
}

#[derive(Debug, Default)]
struct Circuit(HashSet<JunctionBox>);

impl Circuit {
    fn new(jb: JunctionBox) -> Self {
        let mut set = HashSet::new();
        set.insert(jb);
        Circuit(set)
    }
    fn contains(&self, value: &JunctionBox) -> bool {
        self.0.contains(value)
    }

    fn merge(mut self, other: Circuit) -> Circuit {
        self.0.extend(other.0);
        Circuit(self.0)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Default)]
struct Circuits(Vec<Circuit>);

impl Circuits {
    fn connect(&mut self, b1: JunctionBox, b2: JunctionBox) {
        let c1 = if let Some(index) = self.0.iter().position(|c| c.contains(&b1)) {
            self.0.swap_remove(index)
        } else {
            Circuit::new(b1)
        };

        let c2 = if let Some(index) = self.0.iter().position(|c| c.contains(&b2)) {
            self.0.swap_remove(index)
        } else {
            Circuit::new(b2)
        };
        self.0.push(c1.merge(c2));
    }

    fn calc_part1(&self) -> usize {
        let mut tmp: Vec<usize> = self.0.iter().map(|c| c.len()).collect();
        tmp.sort();
        //dbg!(&tmp);
        tmp.iter().rev().take(3).product()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Default)]
struct Problem {
    boxes: Vec<JunctionBox>,
    distances: BTreeMap<i64, (JunctionBox, JunctionBox)>,
}

impl AoCProblem<usize, i64> for Problem {
    fn date() -> Date {
        Date::new(2025, 8).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.boxes = input.parse_lines()?;

        for (b1, b2) in self.boxes.pairs() {
            self.distances.insert(b1.distance_to_squared(&b2), (b1, b2));
        }

        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        let mut circuits = Circuits::default();

        #[cfg(not(test))]
        let iterations = 1000;
        #[cfg(test)]
        let iterations = 10;

        for (b1, b2) in self.distances.values().take(iterations) {
            circuits.connect(*b1, *b2);
        }

        Ok(circuits.calc_part1())
    }

    fn part2(&self) -> Result<i64> {
        let mut circuits = Circuits::default();

        let mut distance = 0;
        for (b1, b2) in self.distances.values() {
            circuits.connect(*b1, *b2);

            if circuits.len() == 1 && circuits.0[0].len() == self.boxes.len() {
                distance = b1.x * b2.x;
                break;
            }
        }

        Ok(distance)
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
        let input = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(40);
        problem.test_part2(25272);
    }
}
