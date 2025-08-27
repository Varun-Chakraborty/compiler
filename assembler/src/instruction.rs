use crate::writer::Writer;
use core::panic;
use std::collections::HashMap;

pub struct Instruction {
    opcode: u32,
    operands: Vec<String>,
    operand_count: u32,
    current_operand: u32,
    opttab: HashMap<&'static str, u32>,
    writer: Writer
}

impl Instruction {
    pub fn new(debug: bool, pretty: bool) -> Self {
        let opttab = HashMap::from([
            ("MOVER", 0),
            ("MOVEM", 1),
            ("ADD", 2),
            ("SUB", 3),
            ("HALT", 4),
            ("IN", 5),
            ("OUT", 6),
        ]);
        return Self {
            opcode: 0,
            operands: Vec::new(),
            operand_count: 0,
            current_operand: 0,
            opttab,
            writer: Writer::new(debug, pretty)
        };
    }

    pub fn set_opcode(&mut self, opcode: &str) {
        if !self.opttab.contains_key(opcode) {
            panic!("Invalid opcode: '{}'", opcode);
        }
        self.opcode = *self.opttab.get(&opcode).unwrap();
        self.operand_count = match self.opcode { 2 | 3 => 3, 0 | 1 => 2, 5 | 6 => 1, _ => 0 };
    }

    pub fn add_operand(&mut self, operand: &str) {
        if operand.is_empty() {
            return;
        }
        if self.operand_count == self.current_operand {
            panic!("Too many operands");
        }
        self.operands.push(operand.to_string());
        self.current_operand += 1;
    }

    pub fn is_empty(&self) -> bool {
        return self.opcode == 0 && self.operand_count == 0;
    }

    pub fn is_incomplete(&self) -> bool {
        return self.operand_count != 0 && self.current_operand < self.operand_count;
    }

    pub fn done(&mut self) {
        if self.is_empty() {
            return;
        }
        if self.is_incomplete() {
            panic!("Incomplete instruction");
        }
        println!("Instruction: Opcode = {}, Operands = {:?}", self.opcode, self.operands);
        self.writer.write(self.opcode, 4);
        for operand in &self.operands {
            if operand.is_empty() {
                continue;
            }
            if operand.chars().all(char::is_numeric) {
                self.writer.write(operand.parse::<u32>().unwrap(), 4);
            } else if operand.chars().nth(0).unwrap() == 'R' && operand[1..].chars().all(char::is_numeric) {
                self.writer.write(operand[1..].parse::<u32>().unwrap(), 2);
            } else {
                panic!("Invalid operand: {}", operand);
            }
        }
        self.writer.new_line();
        self.opcode = 0;
        self.operand_count = 0;
        self.current_operand = 0;
        self.operands.clear();
    }

    pub fn close(&mut self) {
        self.writer.close();
    }
}