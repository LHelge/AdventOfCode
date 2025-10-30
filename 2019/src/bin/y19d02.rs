const YEAR: u16 = 2019;
const DAY: u8 = 2;
use aoc::{intcode::CPU, *};

type ResultType = u64;
type DataType = intcode::CPU;

fn run_program(mut cpu: CPU, noun: u64, verb: u64) -> Result<ResultType> {
    cpu.write(1, noun)?;
    cpu.write(2, verb)?;
    cpu.run()?;
    Ok(cpu.pos0()?)
}

fn parse(input: &str) -> Result<DataType> {
    Ok(intcode::CPU::new(input.parse()?))
}

fn task1(cpu: &DataType) -> Result<ResultType> {
    run_program(cpu.clone(), 12, 2)
}

fn task2(cpu: &DataType) -> Result<ResultType> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(cpu.clone(), noun, verb)? == 19690720 {
                return Ok(100 * noun + verb);
            }
        }
    }
    Err(AoCError::BadInput)
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
        let mut cpu = CPU::new("1,9,10,3,2,3,11,0,99,30,40,50".parse().unwrap());
        cpu.run().unwrap();
        assert_eq!(cpu.pos0().unwrap(), 3500);
    }
}
