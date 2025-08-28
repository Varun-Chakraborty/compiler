use crate::writer::Writer;
use core::panic;
use std::collections::HashMap;
use isa::{OptTab};

pub struct Instruction {
    opcode: u32,
    operands: Vec<String>,
    operand_count: u32,
    current_operand: u32,
    opttab: OptTab,
    symtab: HashMap<String, u32>,
    is_empty: bool,
    is_there_label: bool,
    label: String,
    location_counter: u32,
    writer: Writer
}

impl Instruction {
    pub fn new(debug: bool, pretty: bool) -> Self {
        return Self {
            opcode: 0,
            operands: Vec::new(),
            operand_count: 0,
            current_operand: 0,
            location_counter: 0,
            is_empty: true,
            is_there_label: false,
            label: String::new(),
            opttab: OptTab::clone(),
            symtab: HashMap::new(),
            writer: Writer::new(debug, pretty)
        };
    }

    pub fn add_token(&mut self, token: String) {
        if self.is_empty {
            // if the instruction is empty, the token must be a label or an opcode
            if token.ends_with(":") {
                if !self.is_there_label {
                    self.is_there_label = true;
                    self.add_label(&token[0..token.len()-1]);
                    return;   
                }
                panic!("A statement can only have one label");
            }
            self.set_opcode(&token);
        } else {
            self.add_operand(&token);
        }
    }

    fn add_label(&mut self, label: &str) {
        self.symtab.insert(label.to_string(), self.location_counter);
        self.label = label.to_string();
    }

    fn set_opcode(&mut self, opcode: &str) {
        if opcode.is_empty() {
            return;
        }
        let operation = self.opttab.get_by_operation_name(opcode);
        self.is_empty = false;
        self.opcode = operation.opcode;
        self.operand_count = operation.expected_arguments;
    }

    fn add_operand(&mut self, operand: &str) {
        if operand.is_empty() {
            return;
        }
        if self.operand_count == self.current_operand {
            panic!("Too many operands");
        }
        self.operands.push(operand.to_string());
        self.current_operand += 1;
    }

    fn increment_location_counter(&mut self, by: u32) {
        self.location_counter += by;
    }

    pub fn is_empty(&self) -> bool {
        return self.is_empty;
    }

    pub fn is_incomplete(&self) -> bool {
        return self.operand_count != 0 && self.current_operand < self.operand_count;
    }

    pub fn print_symtab(&self) {
        println!("Symbol Table:");
        println!("{:?}", self.symtab);
    }

    pub fn done(&mut self) {
        if self.is_empty() {
            if self.is_there_label {
                self.symtab.remove(self.label.as_str());
            }
            return;
        }
        if self.is_incomplete() {
            panic!("Incomplete instruction");
        }
        println!("Instruction: Opcode = {}, Operands = {:?}", self.opcode, self.operands);
        self.writer.write(self.opcode, 4);
        self.increment_location_counter(4);
        let mut bits_written = 0;
        for operand in &self.operands {
            if operand.is_empty() {
                continue;
            }
            if operand.chars().all(char::is_numeric) {
                self.writer.write(operand.parse::<u32>().unwrap(), 4);
                bits_written += 4;
            } else if operand.chars().nth(0).unwrap() == 'R' && operand[1..].chars().all(char::is_numeric) {
                self.writer.write(operand[1..].parse::<u32>().unwrap(), 2);
                bits_written += 2;
            } else if self.symtab.contains_key(operand.as_str()) && matches!(self.opcode, 7 | 8 | 9) {
                self.writer.write(*self.symtab.get(operand.as_str()).unwrap(), 8);
                bits_written += 8;
            } else {
                panic!("Invalid operand: {}", operand);
            }
        }
        self.increment_location_counter(bits_written);
        self.writer.new_line();
        self.opcode = 0;
        self.operand_count = 0;
        self.current_operand = 0;
        self.is_empty = true;
        self.operands.clear();
    }

    pub fn close(&mut self) {
        self.writer.close();
    }
}