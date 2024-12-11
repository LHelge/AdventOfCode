const YEAR: u16 = 2024;
const DAY: u8 = 11;

use aoc::*;
use std::num::ParseIntError;

type ResultType = usize;
type DataType = Vec<u64>;

#[memoize::memoize]
fn blink(stone: u64, blinks_left: usize) -> usize {
    if blinks_left == 0 {
        return 1;
    }

    let string = stone.to_string();

    if stone == 0 {
        blink(1, blinks_left - 1)
    } else if string.len() % 2 == 0 {
        let (str1, str2) = string.split_at(string.len() / 2);

        blink(str1.parse().unwrap(), blinks_left - 1)
            + blink(str2.parse().unwrap(), blinks_left - 1)
    } else {
        blink(stone * 2024, blinks_left - 1)
    }
}

fn parse(input: &str) -> Result<DataType> {
    Ok(input
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<std::result::Result<Vec<u64>, ParseIntError>>()?)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|&stone| blink(stone, 25)).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|&stone| blink(stone, 75)).sum())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let input = "125 17";

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(55312));
        assert_eq!(task2, Some(65601038650482));
    }
}
