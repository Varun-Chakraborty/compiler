use isa::{OptSpec};

use crate::memory::Memory;

pub struct Instruction {
    opcode: u32,
    operands: Vec<u32>,
}

impl Instruction {
    pub fn new(memory: &Memory, program_counter: &mut u32) -> Self {
        let optspec = OptSpec::clone();
        let mut pc = *program_counter;
        let opcode = (0..optspec.opcode_bit_count)
            .map(|_| {
                let bit = memory.get(pc);
                pc += 1;
                return bit;
            })
            .fold(0, |acc, x| acc << 1 | x) as u32;

        if !optspec.contains_opcode(opcode) {
            panic!("Invalid opcode: {}", opcode);
        }

        let operands: Vec<u32> = optspec
            .get_by_opcode(&opcode)
            .operands
            .iter()
            .map(|operand| {
                let operand = (0..operand.bit_count)
                    .map(|_| {
                        let bit = memory.get(pc);
                        pc += 1;
                        return bit;
                    })
                    .fold(0, |acc, x| acc << 1 | x) as u32;
                return operand;
            }).collect();

        *program_counter = pc;

        return Self {
            opcode,
            operands,
        };
    }

    pub fn get_opcode(&self) -> u32 {
        return self.opcode;
    }

    pub fn get_operands(&self) -> &Vec<u32> {
        return &self.operands;
    }
}
