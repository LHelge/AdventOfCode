const YEAR: u16 = 2019;
const DAY: u8 = 4;
use std::ops::RangeInclusive;

use aoc::*;

#[derive(Debug)]
struct Chunk {
    c: char,
    n: usize,
}

trait StringChunker {
    fn chunks(&self) -> Vec<Chunk>;
}

impl StringChunker for String {
    fn chunks(&self) -> Vec<Chunk> {
        let mut chunks = Vec::new();

        let mut chars = self.chars();

        if let Some(first) = chars.next() {
            let mut chunk = Chunk { c: first, n: 1 };

            for c in chars {
                if chunk.c == c {
                    chunk.n += 1;
                } else {
                    chunks.push(chunk);
                    chunk = Chunk { c, n: 1 };
                }
            }

            chunks.push(chunk);
        }

        chunks
    }
}

trait PasswordValidator {
    fn is_valid1(&self) -> bool;
    fn is_valid2(&self) -> bool;
}

impl PasswordValidator for u64 {
    fn is_valid1(&self) -> bool {
        let chunks = self.to_string().chunks();

        for w in chunks.as_slice().windows(2) {
            if !w[0].c.is_numeric() || !w[1].c.is_numeric() || w[0].c > w[1].c {
                return false;
            }
        }

        chunks.iter().any(|c| c.n > 1)
    }

    fn is_valid2(&self) -> bool {
        let chunks = self.to_string().chunks();

        for w in chunks.as_slice().windows(2) {
            if !w[0].c.is_numeric() || !w[1].c.is_numeric() || w[0].c > w[1].c {
                return false;
            }
        }

        chunks.iter().any(|c| c.n == 2)
    }
}

type ResultType = usize;
type DataType = RangeInclusive<u64>;

fn parse(input: &str) -> Result<DataType> {
    let (from, to) = input.split_once('-').ok_or(AoCError::BadInput)?;
    let from = from.parse().map_err(|_| AoCError::BadInput)?;
    let to = to.parse().map_err(|_| AoCError::BadInput)?;
    Ok(from..=to)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data.clone().filter(|pin| pin.is_valid1()).count())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data.clone().filter(|pin| pin.is_valid2()).count())
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer_input("165432-707912")?;

    println!("Advent of Code {YEAR} day {DAY}");
    println!("-------------------------");
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"111111-111111"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1));
        assert_eq!(task2, Some(0));
    }

    #[test]
    fn example2() {
        let input = r#"223450-223450"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(0));
        assert_eq!(task2, Some(0));
    }

    #[test]
    fn example3() {
        let input = r#"123789-123789"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(0));
        assert_eq!(task2, Some(0));
    }

    #[test]
    fn example4() {
        let input = r#"112233-112233"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1));
        assert_eq!(task2, Some(1));
    }

    #[test]
    fn example5() {
        let input = r#"123444-123444"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1));
        assert_eq!(task2, Some(0));
    }

    #[test]
    fn example6() {
        let input = r#"111122-111122"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1));
        assert_eq!(task2, Some(1));
    }
}
