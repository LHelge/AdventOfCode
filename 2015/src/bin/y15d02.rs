use aoc::{AoCError, AoCInput};
use std::str::FromStr;

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Present {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('x');

        let l = parts.next().ok_or(AoCError::BadInput)?.parse()?;
        let w = parts.next().ok_or(AoCError::BadInput)?.parse()?;
        let h = parts.next().ok_or(AoCError::BadInput)?.parse()?;

        Ok(Present { l, w, h })
    }
}

impl Present {
    fn paper_required(&self) -> u32 {
        let side1_area = self.l * self.w;
        let side2_area = self.w * self.h;
        let side3_area = self.h * self.l;

        let smallest_side_area = side1_area.min(side2_area).min(side3_area);

        2 * side1_area + 2 * side2_area + 2 * side3_area + smallest_side_area
    }

    fn ribbon_required(&self) -> u32 {
        let side1_circumference = 2 * (self.l + self.w);
        let side2_circumference = 2 * (self.w + self.h);
        let side3_circumference = 2 * (self.h + self.l);

        let smallest_side_circumference = side1_circumference
            .min(side2_circumference)
            .min(side3_circumference);

        let volume = self.l * self.w * self.h;

        smallest_side_circumference + volume
    }
}

fn solve_task(input: &str) -> (u32, u32) {
    let presents: Vec<Present> = input
        .lines()
        .map(|line| line.trim().parse().expect("Bad input"))
        .collect();

    let task1 = presents.iter().map(|p| p.paper_required()).sum();
    let task2 = presents.iter().map(|p| p.ribbon_required()).sum();

    (task1, task2)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2015, 2)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2015d02 {
    use super::*;

    #[test]
    fn examples() {
        let (example1, example2) = solve_task("2x3x4");
        assert_eq!(example1, 58);
        assert_eq!(example2, 34);

        let (example1, example2) = solve_task("1x1x10");
        assert_eq!(example1, 43);
        assert_eq!(example2, 14);
    }
}
