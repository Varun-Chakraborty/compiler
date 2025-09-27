use crate::memory::{Memory, MemoryError};
use isa::{OptSpec, OptSpecError};

#[derive(Debug, thiserror::Error)]
pub enum InstructionError {
    #[error("{0}")]
    MemoryError(#[from] MemoryError),
    #[error("Invalid opcode: {0}")]
    InvalidOpcode(u32),
    #[error("{0}")]
    OperationError(#[from] OptSpecError),
}

pub struct Instruction {
    opcode: u32,
    operands: Vec<u32>,
}

fn get_bits(memory: &Memory, mut start: u32, bits_count: u32) -> Result<u32, InstructionError> {
    let mut value: u32 = 0;
    for _ in 0..bits_count {
        let byte = memory.get(start / 8)?;
        let bit = (byte >> (7 - start % 8)) & 1;
        value = (value << 1) | bit as u32;
        start += 1;
    }
    Ok(value)
}

impl Instruction {
    pub fn new(memory: &Memory, pc: &mut u32) -> Result<Self, InstructionError> {
        let optspec = OptSpec::clone();
        let opcode = get_bits(memory, *pc, 4)?;
        *pc += 4;

        if !optspec.contains_opcode(opcode) {
            return Err(InstructionError::InvalidOpcode(opcode));
        }

        let operands = optspec.get_by_opcode(&opcode)?.operands.iter().fold(
            Ok(Vec::new()),
            |acc: Result<Vec<u32>, InstructionError>, operand_spec| {
                let mut acc = acc?;
                let operand = get_bits(memory, *pc, operand_spec.bit_count as u32)?;
                *pc += operand_spec.bit_count as u32;
                acc.push(operand);
                Ok(acc)
            },
        )?;

        Ok(Self { opcode, operands })
    }

    pub fn get_opcode(&self) -> u32 {
        return self.opcode;
    }

    pub fn get_operands(&self) -> &Vec<u32> {
        return &self.operands;
    }
}
