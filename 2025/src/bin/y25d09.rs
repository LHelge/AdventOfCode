use aoc::{problem::*, utils::*, *};
use std::collections::HashSet;

#[derive(Default)]
struct Problem {
    tiles: Vec<Position>,
}

impl AoCProblem<usize, usize> for Problem {
    fn date() -> Date {
        Date::new(2025, 9).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.tiles = input.parse_lines()?;
        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        let mut largest = 0;

        for (t1, t2) in self.tiles.pairs() {
            let size = (t1.x.abs_diff(t2.x) + 1) * (t1.y.abs_diff(t2.y) + 1);

            largest = largest.max(size);
        }

        Ok(largest)
    }

    fn part2(&self) -> Result<usize> {
        let mut red_green: HashSet<Position> = HashSet::new();

        let mut xmin = usize::MAX;
        let mut xmax = 0;
        let mut ymin = usize::MAX;
        let mut ymax = 0;

        for w in self.tiles.as_slice().windows(2) {
            let t1 = w[0];
            let t2 = w[1];
            red_green.insert(t1);
            red_green.insert(t2);
            if t1.x == t2.x {
                let x = t1.x;
                for y in t1.y.min(t2.y)..=t1.y.max(t2.y) {
                    xmin = xmin.min(x);
                    ymin = ymin.min(y);
                    xmax = xmax.max(x);
                    ymax = ymax.max(y);
                    let t3 = Position::new(x, y);
                    red_green.insert(t3);
                }
            } else if t1.y == t2.y {
                let y = t1.y;
                for x in t1.x.min(t2.x)..=t1.x.max(t2.x) {
                    xmin = xmin.min(x);
                    ymin = ymin.min(y);
                    xmax = xmax.max(x);
                    ymax = ymax.max(y);
                    let t3 = Position::new(x, y);
                    red_green.insert(t3);
                }
            } else {
                Err(AoCError::BadInput)?
            }
        }

        for y in 0..=ymax {
            let mut set = false;
            for x in 0..=xmax {
                let p = Position::new(x, y);
                if red_green.contains(&p) {
                    set = !set;
                }

                if set {
                    red_green.insert(p);
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }

        let mut largest = 0;

        'outer: for (t1, t2) in self.tiles.pairs() {
            let size = (t1.x.abs_diff(t2.x) + 1) * (t1.y.abs_diff(t2.y) + 1);

            for x in t1.x.min(t2.x)..=t1.x.max(t2.x) {
                for y in t1.y.min(t2.y)..=t1.y.max(t2.y) {
                    let p = Position::new(x, y);
                    if !red_green.contains(&p) {
                        continue 'outer;
                    }
                }
            }
            largest = largest.max(size);
        }

        Ok(largest)
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
        let input = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(50);
        problem.test_part2(24);
    }
}
