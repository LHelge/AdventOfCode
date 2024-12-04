use aoc::*;

type ResultType = usize;
type DataType = ();

fn parse(_input: &str) -> Result<DataType> {
    Ok(())
}

fn task1(_data: &DataType) -> Result<ResultType> {
    Err(AoCError::Unsolved)
}

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
mod y2024d02 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#""#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, None);
        assert_eq!(task2, None);
    }
}
