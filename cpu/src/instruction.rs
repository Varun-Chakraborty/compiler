use std::{collections::HashMap, vec};
use std::{sync::LazyLock};
use crate::memory::Memory;

pub struct Instruction {
    opcode: u32,
    program_counter: u32,
    operands: Vec<u32>
}

static SYMBOL_TABLE: LazyLock<HashMap<u32, u32>> = LazyLock::new(|| {
    let mut st = HashMap::new();
    st.insert(0, 2);
    st.insert(1, 2);
    st.insert(2, 3);
    st.insert(3, 3);
    st.insert(4, 0);
    st.insert(5, 1);
    st.insert(6, 1);
    return st;
});
        

impl Instruction {
    pub fn new(memory: &Memory, program_counter: &mut u32) -> Self {
        let mut program_counter = *program_counter;
        let opcode = (0..4).map(|_| {
            let bit = memory.get(program_counter);
            program_counter += 1;
            return bit;
        }).fold(0, |acc, x| acc << 1 | x) as u32;

        if !SYMBOL_TABLE.contains_key(&opcode) {
            panic!("Invalid opcode: {}", opcode);
        }

        let operand_count = *SYMBOL_TABLE.get(&opcode).unwrap();

        let mut operands: Vec<u32> = vec![];
        for i in 0..operand_count {
            if i == 0 || (i == 1 && (opcode == 2 || opcode == 3)) {
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

        if *SYMBOL_TABLE.get(&opcode).unwrap() as usize != operands.len() {
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