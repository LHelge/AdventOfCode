const YEAR: u16 = 2024;
const DAY: u8 = 1;

use aoc::*;

type ResultType = u64;
type DataType = (Vec<u64>, Vec<u64>);

fn parse_line(line: &str) -> (u64, u64) {
    let (a, b) = line.split_once(' ').unwrap();
    (a.trim().parse().unwrap(), b.trim().parse().unwrap())
}

fn parse(input: &str) -> Result<DataType> {
    let vecs = input.lines().map(parse_line).unzip();

    Ok((vecs.0, vecs.1))
}

fn task1(data: &DataType) -> Result<ResultType> {
    let (mut list1, mut list2) = data.clone();

    list1.sort();
    list2.sort();

    Ok(list1
        .iter()
        .zip(list2.iter())
        .map(|(&l1, &l2)| l1.abs_diff(l2))
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    let (list1, list2) = data;

    Ok(list1
        .iter()
        .map(|&l1| list2.iter().filter(|&&l2| l1 == l2).count() as u64 * l1)
        .sum())
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
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(11));
        assert_eq!(task2, Some(31));
    }
}
