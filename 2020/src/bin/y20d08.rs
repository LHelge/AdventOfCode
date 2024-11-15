use std::collections::HashSet;

use aoc::AoCInput;

#[derive(Debug, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(isize),
    Nop,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (instruction, data) = value.split_once(' ').unwrap();

        match instruction {
            "acc" => Instruction::Acc(data.parse().unwrap()),
            "jmp" => Instruction::Jmp(data.parse().unwrap()),
            "nop" => Instruction::Nop,
            _ => panic!("Unknown instruction: {instruction}"),
        }
    }
}

struct Cpu {
    program: Vec<Instruction>,
    pc: usize,
    acc: i64,
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        match self.program[self.pc] {
            Instruction::Acc(data) => {
                self.acc += data;
                self.pc += 1;
            }
            Instruction::Jmp(data) => {
                self.pc = self.pc.wrapping_add_signed(data);
            }
            Instruction::Nop => {
                self.pc += 1;
            }
        }
    }
}

fn task1(program: &[Instruction]) -> i64 {
    let mut cpu = Cpu::new(Vec::from(program));

    let mut visited = HashSet::new();

    loop {
        if !visited.insert(cpu.pc) {
            return cpu.acc;
        }
        cpu.step();
    }
}

fn solve_task(input: &str) -> (i64, usize) {
    let program = input
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    let task1 = task1(&program);

    (task1, 0)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2020, 8)
        .expect("Could not fetch input");

    let (task1, task2) = solve_task(&input);

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}

#[cfg(test)]
mod y2020d08 {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

        let (example1, _example2) = solve_task(input);
        assert_eq!(example1, 5);
    }
}
