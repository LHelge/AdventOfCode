const YEAR: u16 = 2025;
const DAY: u8 = 3;

use aoc::*;

type ResultType = u64;
type DataType = Vec<Vec<u8>>;

fn parse(input: &str) -> Result<DataType> {
    Ok(input
        .lines()
        .map(|l| l.bytes().map(|c| c - b'0').collect())
        .collect())
}

fn get_max_joltage(bank: &[u8], num: usize) -> u64 {
    if num == 0 {
        return 0;
    }

    let mut joltage = 0;

    for digit in (0..=9).rev() {
        if let Some(index) = bank[..=(bank.len() - num)].iter().position(|&d| d == digit) {
            joltage += digit as u64 * 10u64.pow(num as u32 - 1);
            joltage += get_max_joltage(&bank[(index + 1)..], num - 1);
            break;
        }
    }

    joltage
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|bank| get_max_joltage(bank, 2)).sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.iter().map(|bank| get_max_joltage(bank, 12)).sum())
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
        //assert_eq!(task2, None);
        assert_eq!(task2, Some(3121910778619));
    }
}
