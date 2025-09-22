use crate::writer::Writer;
use isa::{OperandSpec, OptSpec, OptSpecError};
use regex::Regex;
use std::{collections::HashMap, io};

#[derive(Debug, thiserror::Error)]
pub enum InstructionError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Regex Compilation error: {0}")]
    RegexCompilation(#[from] regex::Error),
    #[error("Operand {0} does not match regex {1}")]
    RegexMismatch(String, String),
    #[error("Unable to parse the token: {0}")]
    ParseInt(String),
    #[error("A statement can only have one label, {0} is being interpreted as a label")]
    TooManyLabels(String),
    #[error("Too many operands")]
    TooManyOperands(),
    #[error("Too few operands, expected {expected} but got {got} operands\n\tat line: {line}")]
    TooFewOperands {
        expected: usize,
        got: usize,
        line: String,
    },
    #[error("{0} is not a valid operation name")]
    OperationName(#[from] OptSpecError),
    #[error("Symbol {0} not found")]
    MissingSymbol(String),
    #[error("Parsing error: {0}")]
    ParseError(String),
    #[error("Opcode is missing")]
    OpcodeMissing(),
}

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
    symtab: &'a mut HashMap<String, u32>,
    debug: bool,
    line: String,
}

impl<'a> Instruction<'a> {
    pub fn new(
        writer: &'a mut Writer,
        location_counter: &'a mut u32,
        symtab: &'a mut HashMap<String, u32>,
        debug: bool,
    ) -> Self {
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
            symtab,
            debug,
            line: String::new(),
        };
    }

    pub fn add_token(&mut self, token: String) -> Result<(), InstructionError> {
        if token.trim().is_empty() {
            return Ok(());
        }

        if self.is_empty {
            // if the instruction is empty, the token must be a label or an opcode
            if token.ends_with(":") {
                if !self.is_there_label {
                    self.is_there_label = true;
                    self.add_label(&token[0..token.len() - 1]);
                    return Ok(());
                }
                return Err(InstructionError::TooManyLabels(
                    token[0..token.len() - 1].into(),
                ));
            }
            self.set_opcode(token)?;
        } else {
            self.add_operand(token)?;
        }
        Ok(())
    }

    fn add_label(&mut self, label: &str) {
        self.symtab
            .insert(label.to_string(), *self.location_counter);
        self.label = label.to_string();
    }

    fn set_opcode(&mut self, operation_name: String) -> Result<(), InstructionError> {
        let operation = self.optspec.get_by_operation_name(&operation_name)?;
        self.is_empty = false;
        self.operation_name = operation.operation_name;
        self.opcode = operation.opcode;
        self.expected_operands = operation.operands;
        Ok(())
    }

    fn add_operand(&mut self, operand: String) -> Result<(), InstructionError> {
        if !self.is_empty && self.operands.len() >= self.expected_operands.len() {
            return Err(InstructionError::TooManyOperands());
        }
        self.operands.push(operand);
        Ok(())
    }

    fn increment_location_counter(&mut self, by: u32) {
        *self.location_counter += by;
    }

    pub fn is_empty(&self) -> bool {
        return self.is_empty;
    }

    pub fn print_instruction(&self) {
        println!(
            "Instruction: Opcode = {}, Operands = {:?}",
            self.opcode, self.operands
        );
    }

    pub fn parse(&mut self, line: &str) -> Result<(), InstructionError> {
        self.line = line.to_string();
        let (opcode, operands) = line
            .split(';')
            .next()
            .ok_or(InstructionError::ParseError(line.to_string()))?
            .split_once(' ')
            .map(|(s, t)| (s.trim(), t.trim()))
            .ok_or(InstructionError::ParseError(line.to_string()))?;
        if opcode.ends_with(',') || operands.starts_with(',') {
            return Err(InstructionError::OpcodeMissing());
        }
        self.add_token(opcode.into())?;
        operands.split(',').try_for_each(|operand| {
            self.add_token(operand.trim().into())?;
            Ok::<(), InstructionError>(())
        })?;
        Ok(())
    }

    pub fn done(&mut self) -> Result<(), InstructionError> {
        if self.is_empty() {
            if self.is_there_label {
                self.symtab.remove(self.label.as_str());
            }
            return Ok(());
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
            return Err(InstructionError::TooFewOperands {
                expected: self.expected_operands.len(),
                got: self.operands.len(),
                line: self.line.to_string(),
            });
        }

        if self.debug {
            self.print_instruction();
        }

        // write the opcode
        self.writer
            .write(self.opcode, self.optspec.opcode_bit_count as u8)?;
        self.increment_location_counter(self.optspec.opcode_bit_count);

        let mut bits_written = 0;

        // zip operand and the corresponding operand spec
        for (spec, token) in self.expected_operands.iter().zip(self.operands.iter()) {
            let re = Regex::new(spec.operand_regex)?;
            if !re.is_match(&token) {
                return Err(InstructionError::RegexMismatch(
                    token.to_string(),
                    spec.operand_regex.to_string(),
                ));
            }

            // parse data
            let value_to_write = if token.starts_with("R") {
                token[1..]
                    .parse()
                    .map_err(|_| InstructionError::ParseInt(token.to_string()))?
            } else if token.chars().all(char::is_numeric) {
                token
                    .parse::<u32>()
                    .map_err(|_| InstructionError::ParseInt(token.to_string()))?
            } else {
                *self
                    .symtab
                    .get(token)
                    .ok_or(InstructionError::MissingSymbol(token.to_string()))?
            };

            // write it down
            self.writer.write(value_to_write, spec.bit_count as u8)?;
            bits_written += spec.bit_count;
        }

        self.increment_location_counter(bits_written);
        self.writer.new_line()?;
        Ok(())
    }
}
