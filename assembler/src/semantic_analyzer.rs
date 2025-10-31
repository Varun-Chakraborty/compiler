use std::collections::HashMap;

use isa::{OperandType, OptSpec, OptSpecError};
use logger::{Logger, LoggerError};
use regex::Regex;

use crate::{
    instruction::{Instruction, InstructionField, SemanticallyParsedInstruction},
    writer::{Writer, WriterError},
};

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("Regex Compilation error: {0}")]
    RegexCompilation(#[from] regex::Error),
    #[error("Operand {operand} does not match the operand type: {operand_type} which looks like {operand_regex} at line: {line}")]
    OperandDoesNotMatch {
        operand: String,
        operand_type: OperandType,
        operand_regex: String,
        line: String,
    },
    #[error("Unable to parse the token as an integer: {0}")]
    ParseInt(String),
    #[error("Unable to parse the token as a signed 8 bit integer: {0}")]
    NotI8(String),
    #[error("Label {0} already in use")]
    LabelAlreadyInUse(String),
    #[error("Too many operands at line: {0}, expected: {1}, got: {2}")]
    TooManyOperands(String, usize, usize),
    #[error("Too few operands, expected {expected} but got {got} operands\n\tat line: {line}")]
    TooFewOperands {
        expected: usize,
        got: usize,
        line: String,
    },
    #[error("{0}")]
    OperationName(#[from] OptSpecError),
    #[error("Writer error: {0}")]
    WriterError(#[from] WriterError),
    #[error("The label in the statement '{0}' has no name")]
    LabelHasNoName(String),
    #[error("Logger error: {0}")]
    LoggerError(#[from] LoggerError),
}

pub struct SemanticAnalyzer {
    debug: bool,
    optspec: OptSpec,
}

impl SemanticAnalyzer {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            optspec: OptSpec::clone(),
        }
    }

    pub fn pseudo_op(&self, mut instruction: Instruction) -> Result<Instruction, SemanticError> {
        instruction.operands = if let Some(operands) = &instruction.operands {
            match &instruction.operation_name {
                Some(operation_name) => match operation_name.as_str() {
                    "ADD" | "ADDI" | "ADC" | "ADCI" | "SUB" | "SUBI" | "SBC" | "SBCI" | "MULT"
                    | "MULTI" | "AND" | "OR" | "XOR" => {
                        if operands.len() == 2 {
                            Some(vec![
                                operands[0].clone(),
                                operands[0].clone(),
                                operands[1].clone(),
                            ])
                        } else {
                            Some(operands.clone())
                        }
                    }
                    "NOT" => {
                        if operands.len() == 1 {
                            Some(vec![operands[0].clone(), operands[0].clone()])
                        } else {
                            Some(operands.clone())
                        }
                    }
                    _ => Some(operands.clone()),
                },
                None => Some(operands.clone()),
            }
        } else {
            None
        };
        Ok(instruction)
    }

    pub fn patch(
        &mut self,
        label: String,
        writer: &mut Writer,
        tii: &mut HashMap<String, Vec<u32>>,
        location_counter: &mut u32,
        logger: &mut Logger,
    ) -> Result<(), SemanticError> {
        if let Some(addr_to_patch) = tii.remove(&label) {
            logger.log(format!("Patching label: {}", label))?;
            for addr in addr_to_patch.iter() {
                if self.debug {
                    logger.log(format!(
                        "Patching address: {} with: {}",
                        addr, location_counter
                    ))?;
                }
                writer.patch(*addr, *location_counter, 8)?;
            }
        }
        Ok(())
    }

    pub fn analyze(
        &mut self,
        instruction: Instruction,
        line: String,
        symtab: &mut HashMap<String, u32>,
        tii: &mut HashMap<String, Vec<u32>>,
        location_counter: &mut u32,
        writer: &mut Writer,
        logger: &mut Logger,
    ) -> Result<Option<SemanticallyParsedInstruction>, SemanticError> {
        if self.debug {
            Instruction::print_instruction(&instruction, logger)?;
        }

        let instruction = self.pseudo_op(instruction)?;

        if let Some(label) = instruction.label {
            if label.is_empty() {
                return Err(SemanticError::LabelHasNoName(line));
            } else {
                match symtab.contains_key(&label) {
                    true => {
                        return Err(SemanticError::LabelAlreadyInUse(label.to_string()));
                    }
                    false => {
                        symtab.insert(label.to_string(), *location_counter);
                    }
                };
                self.patch(label, writer, tii, location_counter, logger)?;
            }
        }

        let operation = match &instruction.operation_name {
            Some(operation_name) => self.optspec.get_by_operation_name(operation_name)?,
            None => return Ok(None),
        };

        let opcode = operation.opcode;
        let mut location_counter = *location_counter + self.optspec.opcode_bit_count as u32;

        let expected_operands = operation.operands.clone();

        let operands = if let Some(operands) = instruction.operands {
            if operands.len() < expected_operands.len() {
                return Err(SemanticError::TooFewOperands {
                    expected: expected_operands.len(),
                    got: operands.len(),
                    line,
                });
            } else if operands.len() > expected_operands.len() {
                return Err(SemanticError::TooManyOperands(
                    line,
                    expected_operands.len(),
                    operands.len(),
                ));
            } else {
                operands
            }
        } else {
            if expected_operands.len() != 0 {
                return Err(SemanticError::TooFewOperands {
                    expected: expected_operands.len(),
                    got: 0,
                    line,
                });
            } else {
                vec![]
            }
        };

        // zip operand and the corresponding operand spec
        let operands: Result<Vec<InstructionField>, SemanticError> = expected_operands
            .iter()
            .zip(operands.iter())
            .map(|(spec, token)| {
                let re = Regex::new(spec.operand_regex.as_str())?;
                
                // parse data
                match spec.operand_type {
                    OperandType::Register => {
                        if !re.is_match(&token) {
                            return Err(SemanticError::OperandDoesNotMatch {
                                operand: token.clone(),
                                operand_type: OperandType::Register,
                                operand_regex: spec.operand_regex.to_string(),
                                line: line.clone(),
                            });
                        }
                        let value = token[1..]
                            .parse()
                            .map_err(|_| SemanticError::ParseInt(token.clone()))?;
                        let bit_count = spec.bit_count;
                        location_counter += bit_count as u32;
                        Ok(InstructionField { value, bit_count })
                    }
                    OperandType::Constant => {
                        if !re.is_match(&token) {
                            return Err(SemanticError::OperandDoesNotMatch {
                                operand: token.clone(),
                                operand_type: OperandType::Constant,
                                operand_regex: spec.operand_regex.to_string(),
                                line: line.clone(),
                            });
                        }
                        let value = token
                            .parse::<i8>()
                            .map_err(|_| SemanticError::NotI8(token.clone()))?
                            as u8 as u32;
                        let bit_count = spec.bit_count;
                        location_counter += bit_count as u32;
                        Ok(InstructionField { value, bit_count })
                    }
                    OperandType::Memory => {
                        if !re.is_match(&token) {
                            return Err(SemanticError::OperandDoesNotMatch {
                                operand: token.clone(),
                                operand_type: OperandType::Memory,
                                operand_regex: spec.operand_regex.to_string(),
                                line: line.clone(),
                            });
                        }
                        let value = token
                            .parse::<u32>()
                            .map_err(|_| SemanticError::ParseInt(token.clone()))?;
                        let bit_count = spec.bit_count;
                        location_counter += bit_count as u32;
                        Ok(InstructionField { value, bit_count })
                    }
                    OperandType::Label => {
                        if !re.is_match(&token) {
                            return Err(SemanticError::OperandDoesNotMatch {
                                operand: token.clone(),
                                operand_type: OperandType::Label,
                                operand_regex: spec.operand_regex.to_string(),
                                line: line.clone(),
                            });
                        }
                        if let Some(location) = symtab.get(token) {
                            let bit_count = spec.bit_count;
                            location_counter += bit_count as u32;

                            Ok(InstructionField {
                                value: *location,
                                bit_count: spec.bit_count,
                            })
                        } else {
                            tii.entry(token.clone()).or_default().push(location_counter);
                            Ok(InstructionField {
                                value: 0,
                                bit_count: spec.bit_count,
                            })
                        }
                    }
                }
            })
            .collect();

        let operands = operands?;

        Ok(Some(SemanticallyParsedInstruction {
            opcode: InstructionField {
                value: opcode,
                bit_count: self.optspec.opcode_bit_count,
            },
            operands: Some(operands),
        }))
    }
}
