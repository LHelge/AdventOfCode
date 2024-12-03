const YEAR: u16 = 2024;
const DAY: u8 = 3;
use aoc::*;
use nom::{branch::alt, bytes::complete::tag, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Multiply(u64, u64),
    Do,
    Dont,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((mul_instruction, do_instruction, dont_instruction))(input)
}

fn mul_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, _) = tag("(")(input)?;
    let (input, a) = nom::character::complete::u64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = nom::character::complete::u64(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Instruction::Multiply(a, b)))
}

fn do_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn dont_instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

type ResultType = u64;
type DataType = Vec<Instruction>;

fn parse(input: &str) -> Result<DataType> {
    let mut data = vec![];

    // Ugly way to parse all the data
    for i in 0..input.len() {
        if let Ok((_, m)) = instruction(&input[i..]) {
            data.push(m);
        }
    }

    dbg!(&data);

    Ok(data)
}

fn task1(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .filter_map(|i| match i {
            Instruction::Multiply(a, b) => Some(a * b),
            _ => None,
        })
        .sum())
}

fn task2(data: &DataType) -> Result<ResultType> {
    Ok(data
        .iter()
        .fold((true, 0), |(do_mul, acc), i| match (do_mul, i) {
            (true, Instruction::Multiply(a, b)) => (do_mul, acc + a * b),
            (_, Instruction::Do) => (true, acc),
            (_, Instruction::Dont) => (false, acc),
            _ => (do_mul, acc),
        })
        .1)
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
    use nom::multi::many1;

    #[test]
    fn example1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (task1, _task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task1, Some(161));
    }

    #[test]
    fn example2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        let mut solution = Solution::<ResultType, DataType>::new(&parse, &task1, &task2);
        let (_task1, task2) = solution.solve_for_test(input).unwrap();
        assert_eq!(task2, Some(48));
    }

    #[test]
    fn nom_parser() {
        let input = "mul(2,4)";
        let result = mul_instruction(input);
        assert_eq!(result, Ok(("", Instruction::Multiply(2, 4))));

        let input = "do()";
        let result = do_instruction(input);
        assert_eq!(result, Ok(("", Instruction::Do)));

        let input = "don't()";
        let result = dont_instruction(input);
        assert_eq!(result, Ok(("", Instruction::Dont)));

        let input = "mul(2,4)do()don't()";
        let result = many1(instruction)(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec![
                    Instruction::Multiply(2, 4),
                    Instruction::Do,
                    Instruction::Dont
                ]
            ))
        );
    }
}
