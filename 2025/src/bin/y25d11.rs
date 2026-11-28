use aoc::{problem::*, *};
use std::collections::HashMap;

struct Problem {
    devices: HashMap<String, Vec<String>>,
}

impl Problem {
    fn paths_to(&self, from: &str, to: &str, cache: &mut HashMap<String, usize>) -> usize {
        if from == to {
            return 1;
        }
        if let Some(paths) = cache.get(from) {
            return *paths;
        }

        let Some(next) = self.devices.get(from) else {
            return 0;
        };

        let paths = next.iter().map(|out| self.paths_to(out, to, cache)).sum();
        cache.insert(from.to_owned(), paths);

        paths
    }
}

impl AoCProblem for Problem {
    type Solution1 = usize;
    type Solution2 = usize;

    const YEAR: u16 = 2025;
    const DAY: u8 = 11;

    fn parse(input: &str) -> Result<Self> {
        let devices = input
            .lines()
            .map(|line| {
                let (device, outputs) = line.split_once(": ").unwrap();
                let outputs = outputs.split(' ').map(|out| out.to_owned()).collect();
                (device.to_owned(), outputs)
            })
            .collect();
        Ok(Self { devices })
    }

    fn part1(&self) -> Result<usize> {
        Ok(self.paths_to("you", "out", &mut HashMap::new()))
    }

    fn part2(&self) -> Result<usize> {
        let svr_fft = self.paths_to("svr", "fft", &mut HashMap::new());
        let fft_dac = self.paths_to("fft", "dac", &mut HashMap::new());
        let dac_out = self.paths_to("dac", "out", &mut HashMap::new());
        let svr_fft_dac = svr_fft * fft_dac * dac_out;

        let svr_dac = self.paths_to("svr", "dac", &mut HashMap::new());
        let dac_fft = self.paths_to("dac", "fft", &mut HashMap::new());
        let fft_out = self.paths_to("fft", "out", &mut HashMap::new());
        let svr_dac_fft = svr_dac * dac_fft * fft_out;

        Ok(svr_fft_dac + svr_dac_fft)
    }
}

fn main() -> Result<()> {
    Problem::run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples1() -> Result<()> {
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

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part1()?, 5);

        Ok(())
    }

    #[test]
    fn examples2() -> Result<()> {
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

        let problem = Problem::parse(input)?;
        assert_eq!(problem.part2()?, 2);

        Ok(())
    }
}
