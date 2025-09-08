use crate::writer::Writer;
use core::panic;
use std::collections::HashMap;
use isa::{OperandSpec, OptSpec};
use regex::Regex;

pub struct Instruction<'a> {
    operation_name: &'static str,
    opcode: u32,
    operands: Vec<String>,
    expected_operands: &'static [OperandSpec],
    optspec: OptSpec,
    is_empty: bool,
    is_there_label: bool,
    label: String,
    location_counter: &'a mut u32,
    writer: &'a mut Writer,
    symtab: &'a mut HashMap<String, u32>
}

impl<'a> Instruction<'a> {
    pub fn new(writer: &'a mut Writer, location_counter: &'a mut u32, symtab: &'a mut HashMap<String, u32>) -> Self {
        return Self {
            operation_name: "",
            opcode: 0,
            operands: Vec::new(),
            expected_operands: &[],
            is_empty: true,
            is_there_label: false,
            label: String::new(),
            optspec: OptSpec::clone(),
            writer,
            location_counter,
            symtab
        };
    }

    pub fn add_token(&mut self, token: String) {
        if token.trim().is_empty() {
            return;
        }

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
            self.set_opcode(token);
        } else {
            self.add_operand(token);
        }
    }

    fn add_label(&mut self, label: &str) {
        self.symtab.insert(label.to_string(), *self.location_counter);
        self.label = label.to_string();
    }

    fn set_opcode(&mut self, operation_name: String) {
        let operation = self.optspec.get_by_operation_name(&operation_name);
        self.is_empty = false;
        self.operation_name = operation.operation_name;
        self.opcode = operation.opcode;
        self.expected_operands = operation.operands;
    }

    fn add_operand(&mut self, operand: String) {
        if !self.is_empty && self.operands.len() >= self.expected_operands.len() {
            panic!("Too many operands");
        }
        self.operands.push(operand);
    }

    fn increment_location_counter(&mut self, by: u32) {
        *self.location_counter += by;
    }

    pub fn is_empty(&self) -> bool {
        return self.is_empty;
    }

    pub fn print_instruction(&self) {
        println!("Instruction: Opcode = {}, Operands = {:?}", self.opcode, self.operands);
    }

    pub fn done(&mut self) {
        if self.is_empty() {
            if self.is_there_label {
                self.symtab.remove(self.label.as_str());
            }
            return;
        }
        
        match self.operation_name {
            "ADD" | "SUB" | "MULT" | "DIV" => {
                if self.operands.len() == 2 {
                    let operands = self.operands.clone();
                    self.operands.clear();
                    self.operands.push(operands[0].clone());
                    self.operands.push(operands[0].clone());
                    self.operands.push(operands[1].clone());
                }
            }
            _ => (),
        }

        if self.operands.len() != self.expected_operands.len() {
            panic!("Incomplete instruction, expected {} operands, but got {}", self.expected_operands.len(), self.operands.len());
        }

        self.print_instruction();
        
        // write the opcode
        self.writer.write(self.opcode, self.optspec.opcode_bit_count as u8);
        self.increment_location_counter(self.optspec.opcode_bit_count);
        
        let mut bits_written = 0;

        // zip operand and the corresponding operand spec
        for (spec, token) in self.expected_operands.iter().zip(self.operands.iter()) {
            let re = Regex::new(spec.operand_regex).unwrap();
            if !re.is_match(&token) {
                panic!("Operand {} does not match regex {}", token, spec.operand_regex);
            }

            // parse data
            let value_to_write = if token.starts_with("R") {
                token[1..].parse().unwrap()
            } else if token.chars().all(char::is_numeric) {
                token.parse::<u32>().unwrap()
            } else {
                *self.symtab.get(token).expect(&format!("Undefined label: {}", token))
            };

            // write it down
            self.writer.write(value_to_write, spec.bit_count as u8);
            bits_written += spec.bit_count;
        }
        
        self.increment_location_counter(bits_written);
        self.writer.new_line();
    }
}