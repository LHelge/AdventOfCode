use num_enum::TryFromPrimitive;
use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum IntCodeError {
    UnknownOpCode,
    EndOfProgram,
    BadProgram,
}

impl Display for IntCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownOpCode => write!(f, "UnknownOpCode"),
            Self::EndOfProgram => write!(f, "End of program"),
            Self::BadProgram => write!(f, "BadProgram"),
        }
    }
}

impl Error for IntCodeError {}

pub type Result<T> = std::result::Result<T, IntCodeError>;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u64)]
pub enum OpCode {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

#[derive(Debug, Clone)]
pub struct Program(Vec<u64>);

impl FromStr for Program {
    type Err = IntCodeError;

    fn from_str(s: &str) -> Result<Self> {
        let prog = s
            .split(',')
            .map(|i| i.parse().map_err(|_| IntCodeError::BadProgram))
            .collect::<std::result::Result<Vec<u64>, IntCodeError>>()?;

        Ok(Program(prog))
    }
}

impl Program {
    fn op_code(&self, addr: usize) -> Result<OpCode> {
        OpCode::try_from(self.value(addr)?).map_err(|_| IntCodeError::UnknownOpCode)
    }

    fn value(&self, addr: usize) -> Result<u64> {
        Ok(*self.0.get(addr).ok_or(IntCodeError::EndOfProgram)?)
    }

    fn ptr(&self, addr: usize) -> Result<u64> {
        self.value(self.value(addr)? as usize)
    }

    fn store(&mut self, addr: usize, value: u64) -> Result<()> {
        *self.0.get_mut(addr).ok_or(IntCodeError::EndOfProgram)? = value;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CPU {
    program: Program,
    pc: usize,
}

impl CPU {
    pub fn new(program: Program) -> Self {
        Self { program, pc: 0 }
    }

    pub fn step(&mut self) -> Result<()> {
        self.pc += match self.program.op_code(self.pc)? {
            OpCode::Add => self.add()?,
            OpCode::Mul => self.mul()?,
            OpCode::Halt => self.halt()?,
        };

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.halted() {
            self.step()?
        }

        Ok(())
    }

    pub fn halted(&self) -> bool {
        !self
            .program
            .op_code(self.pc)
            .is_ok_and(|op| op != OpCode::Halt)
    }

    pub fn pos0(&self) -> Result<u64> {
        self.program
            .0
            .first()
            .copied()
            .ok_or(IntCodeError::EndOfProgram)
    }

    pub fn write(&mut self, addr: usize, value: u64) -> Result<()> {
        self.program.store(addr, value)
    }

    fn add(&mut self) -> Result<usize> {
        let operand1 = self.program.ptr(self.pc + 1)?;
        let operand2 = self.program.ptr(self.pc + 2)?;
        self.program.store(
            self.program.value(self.pc + 3)? as usize,
            operand1 + operand2,
        )?;
        Ok(4)
    }

    fn mul(&mut self) -> Result<usize> {
        let operand1 = self.program.ptr(self.pc + 1)?;
        let operand2 = self.program.ptr(self.pc + 2)?;
        self.program.store(
            self.program.value(self.pc + 3)? as usize,
            operand1 * operand2,
        )?;
        Ok(4)
    }

    fn halt(&self) -> Result<usize> {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_y19d02_1() {
        let mut cpu = CPU::new("1,0,0,0,99".parse().unwrap());
        cpu.run().unwrap();
        assert_eq!(cpu.program.0, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn examples_y19d02_2() {
        let mut cpu = CPU::new("2,3,0,3,99".parse().unwrap());
        cpu.run().unwrap();
        assert_eq!(cpu.program.0, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn examples_y19d02_3() {
        let mut cpu = CPU::new("2,4,4,5,99,0".parse().unwrap());
        cpu.run().unwrap();
        assert_eq!(cpu.program.0, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn examples_y19d02_4() {
        let mut cpu = CPU::new("1,1,1,4,99,5,6,0,99".parse().unwrap());
        cpu.run().unwrap();
        assert_eq!(cpu.program.0, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
