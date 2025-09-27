use std::collections::HashMap;

use isa::{OptSpec, OptSpecError};
use regex::Regex;

use crate::{
    instruction::{Instruction, InstructionField, SemanticallyParsedInstruction},
    writer::{Writer, WriterError},
};

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("Regex Compilation error: {0}")]
    RegexCompilation(#[from] regex::Error),
    #[error("Operand {0} does not match regex {1}")]
    RegexMismatch(String, String),
    #[error("Unable to parse the token: {0}")]
    ParseInt(String),
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
            match instruction.operation_name.as_str() {
                "ADD" | "SUB" | "MULT" | "DIV" => {
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
                _ => Some(operands.clone()),
            }
        } else {
            None
        };
        instruction.set_operation_name(instruction.operation_name.clone());
        Ok(instruction)
    }

    pub fn patch(
        &mut self,
        label: String,
        writer: &mut Writer,
        tii: &mut HashMap<String, Vec<u32>>,
        location_counter: &mut u32,
    ) -> Result<(), SemanticError> {
        if let Some(addr_to_patch) = tii.remove(&label) {
            println!("Patching label: {}", label);
            for addr in addr_to_patch.iter() {
                if self.debug {
                    println!("Patching address: {} with: {}", addr, location_counter);
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
    ) -> Result<SemanticallyParsedInstruction, SemanticError> {
        if self.debug {
            Instruction::print_instruction(&instruction, None);
        }

        let instruction = self.pseudo_op(instruction)?;

        let operation = self
            .optspec
            .get_by_operation_name(&instruction.operation_name)?;


        let opcode = operation.opcode;

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
                self.patch(label, writer, tii, location_counter)?;
            }
        }

        // zip operand and the corresponding operand spec
        let operands: Result<Vec<InstructionField>, SemanticError> = expected_operands
            .iter()
            .zip(operands.iter())
            .map(|(spec, token)| {
                let re = Regex::new(spec.operand_regex.as_str())?;
                if !re.is_match(&token) {
                    return Err(SemanticError::RegexMismatch(
                        token.clone(),
                        spec.operand_regex.to_string(),
                    ));
                }

                // parse data
                if token.starts_with("R") {
                    let value = token[1..]
                        .parse()
                        .map_err(|_| SemanticError::ParseInt(token.clone()))?;
                    Ok(InstructionField {
                        value,
                        bit_count: spec.bit_count,
                    })
                } else if token.chars().all(char::is_numeric) {
                    let value = token
                        .parse::<u32>()
                        .map_err(|_| SemanticError::ParseInt(token.clone()))?;
                    Ok(InstructionField {
                        value,
                        bit_count: spec.bit_count,
                    })
                } else {
                    if let Some(location) = symtab.get(token) {
                        Ok(InstructionField {
                            value: *location,
                            bit_count: spec.bit_count,
                        })
                    } else {
                        tii.entry(token.clone())
                            .or_default()
                            .push(*location_counter);
                        Ok(InstructionField {
                            value: 0,
                            bit_count: spec.bit_count,
                        })
                    }
                }
            })
            .collect();

        let operands = operands?;

        Ok(SemanticallyParsedInstruction {
            opcode: InstructionField {
                value: opcode,
                bit_count: self.optspec.opcode_bit_count,
            },
            operands: Some(operands),
        })
    }
}
