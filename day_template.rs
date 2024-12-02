use aoc::*;

type ResultType = usize;
type DataType = Vec<u64>;

fn parse(input: &str) -> Result<DataType> {
    Ok(input.split(' ').map(|l| l.parse().unwrap()).collect())
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data[0] as usize)
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data[1] as usize)
}

fn main() -> Result<()> {
    let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
    solution.solve_for_answer(YEAR, DAY)?;
    println!("{solution}");

    Ok(())
}

#[cfg(test)]
mod y2024d02 {
    use super::*;

    #[test]
    fn examples() {
        let input = r#"1 2"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(1));
        assert_eq!(task2, Some(2));
    }
}
