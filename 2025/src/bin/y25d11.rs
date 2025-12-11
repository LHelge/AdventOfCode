use std::collections::{HashMap, HashSet};

use aoc::{problem::*, *};

#[derive(Default)]
struct Problem {
    devices: HashMap<String, Vec<String>>,
}

impl Problem {
    fn paths_to(&self, from: &str, to: &str, mut visited: HashSet<String>) -> usize {
        if from == to {
            return 1;
        }
        if visited.contains(from) {
            return 0;
        }
        visited.insert(from.to_owned());

        self.devices
            .get(from)
            .unwrap()
            .iter()
            .map(|out| self.paths_to(out, to, visited.clone()))
            .sum()
    }

    fn paths_to2(
        &self,
        from: &str,
        to: &str,
        mut visited: HashSet<String>,
        cache: &mut HashMap<String, usize>,
    ) -> usize {
        if from == to {
            print!("Found path through: {:?}", visited);
            return if visited.contains("fft") && visited.contains("dac") {
                println!("Visited dac & fft");
                1
            } else {
                println!("Did not visit dac & fft");
                0
            };
        }

        //if visited.contains(from) && !visited.contains("fft") && !visited.contains("dac") {
        //    return 0;
        //}
        visited.insert(from.to_owned());
        print!("{from} ->");

        let paths = self
            .devices
            .get(from)
            .unwrap()
            .iter()
            .map(|out| {
                if let Some(paths) = cache.get(out) {
                    println!("Cache hit on {out}={paths}");
                    return *paths;
                };
                self.paths_to2(out, to, visited.clone(), cache)
            })
            .sum();

        cache.insert(to.to_owned(), paths);

        paths
    }
}

impl AoCProblem<usize, usize> for Problem {
    fn date() -> Date {
        Date::new(2025, 11).unwrap()
    }

    fn parse(&mut self, input: &str) -> Result<()> {
        self.devices = input
            .lines()
            .map(|line| {
                let (device, outputs) = line.split_once(": ").unwrap();
                let outputs = outputs.split(' ').map(|out| out.to_owned()).collect();
                (device.to_owned(), outputs)
            })
            .collect();
        Ok(())
    }

    fn part1(&self) -> Result<usize> {
        Ok(self.paths_to("you", "out", HashSet::new()))
    }

    fn part2(&self) -> Result<usize> {
        let mut cache = HashMap::new();
        Ok(self.paths_to2("svr", "out", HashSet::new(), &mut cache))
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
    fn examples1() {
        let input = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part1(5);
    }

    #[test]
    fn examples2() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

        let mut problem = Problem::default();
        problem.parse(input).unwrap();
        problem.test_part2(2);
    }
}
