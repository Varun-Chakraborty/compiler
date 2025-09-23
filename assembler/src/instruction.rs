use crate::writer::{Writer, WriterError};
use isa::{OperandSpec, OptSpec, OptSpecError};
use regex::Regex;
use std::{collections::HashMap, io, mem};

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
    #[error("Label {0} already in use")]
    LabelAlreadyInUse(String),
    #[error("Too many operands at line: {0}")]
    TooManyOperands(String),
    #[error("Too few operands, expected {expected} but got {got} operands\n\tat line: {line}")]
    TooFewOperands {
        expected: usize,
        got: usize,
        line: String,
    },
    #[error("{0}")]
    OperationName(#[from] OptSpecError),
    #[error("Parsing error: {message}")]
    ParseError { message: String },
    #[error("Opcode is missing at line: {line}")]
    OpcodeMissing { line: String },
    #[error("{0}")]
    WriterError(#[from] WriterError),
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
    tii: &'a mut HashMap<String, u32>,
}

impl<'a> Instruction<'a> {
    pub fn new(
        writer: &'a mut Writer,
        location_counter: &'a mut u32,
        symtab: &'a mut HashMap<String, u32>,
        debug: bool,
        tii: &'a mut HashMap<String, u32>,
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
            tii,
        };
    }

    fn add_label(&mut self, label: &str) -> Result<(), InstructionError> {
        match self.symtab.contains_key(label) {
            true => {
                return Err(InstructionError::LabelAlreadyInUse(label.to_string()));
            }
            false => {
                if !label.is_empty() {
                    if self.is_there_label {
                        return Err(InstructionError::TooManyLabels(label.to_string()));
                    } else {
                        self.symtab
                            .insert(label.to_string(), *self.location_counter);
                        self.label = label.to_string();
                    }
                }
            }
        };
        // check if tii contains this label
        let is_label_in_tii = self.tii.contains_key(label);
        if !is_label_in_tii {
            return Ok(());
        }
        let location = self.tii.get(label).unwrap();
        self.writer.patch(*location, *self.location_counter, 8)?;
        self.tii.remove(label);
        Ok(())
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
            return Err(InstructionError::TooManyOperands(self.line.clone()));
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
        let instruction = line
            .split(';')
            .next()
            .ok_or(InstructionError::ParseError {
                message: format!(
                    "Unable to parse instruction out of line: {}",
                    line.to_string()
                ),
            })?
            .trim();
        if let Some((label, instruction)) = instruction.split_once(':') {
            if instruction.is_empty() {
                return Err(InstructionError::ParseError {
                    message: format!("The label cannot be empty: {}", line.to_string()),
                });
            }
            self.add_label(label.trim())?;
            return self.parse(instruction.trim());
        }

        if !instruction.contains(' ') {
            self.set_opcode(instruction.to_string())?;
            return Ok(());
        }

        let (opcode, operands) = instruction
            .split_once(' ')
            .map(|(s, t)| (s.trim(), t.trim()))
            .ok_or(InstructionError::ParseError {
                message: format!(
                    "Unable to parse opcode and operands out of line: {}",
                    line.to_string()
                ),
            })?;
        if opcode.ends_with(',') || operands.starts_with(',') {
            return Err(InstructionError::OpcodeMissing {
                line: line.to_string(),
            });
        }
        self.set_opcode(opcode.into())?;
        operands.split(',').try_for_each(|operand| {
            self.add_operand(operand.trim().into())?;
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
                    let operands = mem::take(&mut self.operands);
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
                line: self.line.clone(),
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
                    token.clone(),
                    spec.operand_regex.to_string(),
                ));
            }

            // parse data
            let value_to_write = if token.starts_with("R") {
                token[1..]
                    .parse()
                    .map_err(|_| InstructionError::ParseInt(token.clone()))?
            } else if token.chars().all(char::is_numeric) {
                token
                    .parse::<u32>()
                    .map_err(|_| InstructionError::ParseInt(token.clone()))?
            } else {
                let label = self.symtab.get(token);
                if label.is_none() {
                    self.tii.insert(token.clone(), *self.location_counter);
                    0
                } else {
                    *label.unwrap()
                }
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
