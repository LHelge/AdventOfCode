const YEAR: u16 = 2025;
const DAY: u8 = 3;
use std::ops::Index;

use aoc::*;

type ResultType = u32;
type DataType = Vec<Vec<u8>>;

fn parse(input: &str) -> Result<DataType> {
    Ok(input
        .lines()
        .map(|l| l.bytes().map(|c| c - '0' as u8).collect())
        .collect())
}

fn get_max_joltage(bank: &[u8], num: usize) -> u32 {
    if num == 0 {
        return 0;
    }

    let mut joltage = 0;

    for digit in (0..=9).rev() {
        if let Some(index) = bank[..(bank.len() - num)].iter().position(|&d| d == digit)
            && bank.len() - index >= num
        {
            joltage += digit as u32 * 10u32.pow(num as u32 - 1);
            joltage += get_max_joltage(&bank[(index + 1)..], num - 1);
            break;
        }
    }

    joltage
}

fn task1(data: &DataType) -> Result<ResultType> {
    let mut joltage = 0;

    for bank in data {
        let bank_joltage = get_max_joltage(bank, 2);
        println!("Bank joltage: {bank_joltage}");
        joltage += bank_joltage;
    }

    Ok(joltage)
}

//fn task1(data: &DataType) -> Result<ResultType> {
//    let mut joltage = 0;
//
//    for bank in data {
//        for digit in (0..=9).rev() {
//            if let Some(index) = bank.iter().position(|d| *d == digit) {
//                if let Some(max) = bank.iter().skip(index + 1).max() {
//                    let bank_joltage = digit * 10 + max;
//                    //println!("Bank joltage: {bank_joltage}");
//                    joltage += bank_joltage as u32;
//                    break;
//                }
//            }
//        }
//    }
//
//    Ok(joltage)
//}

fn task2(_data: &DataType) -> Result<ResultType> {
    Err(AoCError::Unsolved)
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
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(357));
        assert_eq!(task2, None);
        //assert_eq!(task2, Some(3121910778619));
    }
}
