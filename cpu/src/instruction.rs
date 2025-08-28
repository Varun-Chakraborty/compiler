use std::{vec};
use isa::OptTab;

use crate::memory::Memory;

pub struct Instruction {
    opcode: u32,
    program_counter: u32,
    operands: Vec<u32>,
}

impl Instruction {
    pub fn new(memory: &Memory, program_counter: &mut u32) -> Self {
        let opttab = OptTab::clone();
        let mut program_counter = *program_counter;
        let opcode = (0..4).map(|_| {
            let bit = memory.get(program_counter);
            program_counter += 1;
            return bit;
        }).fold(0, |acc, x| acc << 1 | x) as u32;

        if !opttab.contains_opcode(opcode) {
            panic!("Invalid opcode: {}", opcode);
        }

        let operand_count = opttab.get_by_opcode(&opcode).expected_arguments as usize;

        let mut operands: Vec<u32> = vec![];
        for i in 0..operand_count {
            if opcode == 7 || opcode == 8 || opcode == 9 {
                let operand = (0..8).map(|_| {
                    let bit = memory.get(program_counter);
                    program_counter += 1;
                    return bit;
                }).fold(0, |acc, x| acc << 1 | x) as u32;
                operands.push(operand);
            } else if i == 0 || (i == 1 && (opcode == 2 || opcode == 3 || opcode == 10)) {
                let operand = (0..2).map(|_| {
                    let bit = memory.get(program_counter);
                    program_counter += 1;
                    return bit;
                }).fold(0, |acc, x| acc << 1 | x) as u32;
                operands.push(operand);       
            } else {
                let operand = (0..4).map(|_| {
                    let bit = memory.get(program_counter);
                    program_counter += 1;
                    return bit;
                }).fold(0, |acc, x| acc << 1 | x) as u32;
                operands.push(operand);
            }
        }

        if operand_count != operands.len() {
            panic!("Invalid Instruction");
        }
        
        return Self {
            opcode,
            program_counter,
            operands
        };
    }

    pub fn get_opcode(&self) -> u32 {
        return self.opcode;
    }

    pub fn get_operands(&self) -> &Vec<u32> {
        return &self.operands;
    }

    pub fn get_program_counter(&self) -> u32 {
        return self.program_counter;
    }
}