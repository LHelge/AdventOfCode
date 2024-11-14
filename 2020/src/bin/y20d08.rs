use aoc::AoCInput;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (instruction, data) = value.split_once(' ').unwrap();
        let data = data.parse().unwrap();

        match instruction {
            "acc" => Instruction::Acc(data),
            "jmp" => Instruction::Jmp(data),
            "nop" => Instruction::Nop,
            _ => panic!("Unknown instruction"),
        }
    }
}

struct Cpu {
    program: Vec<Instruction>,
    pc: usize,
    acc: i32,
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            program,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) -> (usize, i32) {
        // TODO

        (self.pc, self.acc)
    }
}

fn solve_task(input: &str) -> (usize, usize) {
    let program = input
        .lines()
        .map(Instruction::from)
        .collect::<Vec<Instruction>>();

    dbg!(program);

    (0, 0)
}

fn main() {
    let input = AoCInput::from_env()
        .get_input(2020, 7)
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
