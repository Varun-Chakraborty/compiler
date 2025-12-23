use isa::{OperandSpec, OperandType, OptSpec, OptSpecError};
use regex::Regex;
use std::collections::HashMap;

use super::instruction::{Instruction, InstructionField, Statement};

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("Regex Compilation error: {0}")]
    RegexCompilation(#[from] regex::Error),
    #[error(
        "Operand {operand} does not match the operand type: {operand_type} (which should look like {operand_regex})"
    )]
    OperandDoesNotMatch {
        operand: String,
        operand_type: OperandType,
        operand_regex: String,
        // line: String
    },
    #[error("Unable to parse the token as an integer: {0}")]
    ParseInt(String),
    #[error("Unable to parse the token as a signed 8 bit integer: {0}")]
    NotI8(String),
    #[error("Label {0} already in use")]
    LabelAlreadyInUse(String),
    #[error("Too many operands expected: {expected}, got: {got}")]
    TooManyOperands {
        expected: usize,
        got: usize,
        // line: String
    },
    #[error("Too few operands, expected {expected} but got {got} operands")]
    TooFewOperands {
        expected: usize,
        got: usize,
        // line: String,
    },
    #[error("{0}")]
    OperationName(#[from] OptSpecError),
    #[error("Operation name is missing")]
    NoOperationName,
    #[error("The label in the statement '{0}' has no name")]
    LabelHasNoName(String),
}

struct TiiEntry {
    instruction_number: usize,
    operand_number: usize,
}

pub struct SemanticParser {
    optspec: OptSpec,
    symtab: HashMap<String, u32>,
    tii: HashMap<String, Vec<TiiEntry>>,
    location_counter: u32,
    instruction_counter: usize,
}

impl SemanticParser {
    pub fn new() -> Self {
        Self {
            optspec: OptSpec::clone(),
            symtab: HashMap::new(),
            tii: HashMap::new(),
            location_counter: 0,
            instruction_counter: 0,
        }
    }

    pub fn normalize(&self, statements: Vec<Statement>) -> Result<Vec<Statement>, SemanticError> {
        statements
            .iter()
            .map(|statement| {
                let mut new_statement = statement.clone();
                new_statement.label = statement.label.clone();
                new_statement.operation_name = statement.operation_name.clone();
                new_statement.operands = if let Some(operands) = new_statement.operands {
                    match &statement.operation_name {
                        Some(operation_name) => match operation_name.as_str() {
                            "ADD" | "ADDI" | "ADC" | "ADCI" | "SUB" | "SUBI" | "SBC" | "SBCI"
                            | "MULT" | "MULTI" | "AND" | "OR" | "XOR" => {
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
                Ok(new_statement)
            })
            .collect()
    }

    pub fn parse_operand(
        &mut self,
        token: String,
        spec: &OperandSpec,
        re: &Regex,
        operand_number: usize,
    ) -> Result<InstructionField, SemanticError> {
        match spec.operand_type {
            OperandType::Register => {
                if !re.is_match(&token) {
                    return Err(SemanticError::OperandDoesNotMatch {
                        operand: token,
                        operand_type: OperandType::Register,
                        operand_regex: spec.operand_regex.to_string(),
                    });
                }
                let value = token[1..]
                    .parse()
                    .map_err(|_| SemanticError::ParseInt(token.to_string()))?;
                let bit_count = spec.bit_count;
                Ok(InstructionField { value, bit_count })
            }
            OperandType::Constant => {
                if !re.is_match(&token) {
                    return Err(SemanticError::OperandDoesNotMatch {
                        operand: token,
                        operand_type: OperandType::Constant,
                        operand_regex: spec.operand_regex.to_string(),
                    });
                }
                let value = token
                    .parse::<i8>()
                    .map_err(|_| SemanticError::NotI8(token))? as u8
                    as u32;
                let bit_count = spec.bit_count;
                Ok(InstructionField { value, bit_count })
            }
            OperandType::Memory => {
                if !re.is_match(&token) {
                    return Err(SemanticError::OperandDoesNotMatch {
                        operand: token,
                        operand_type: OperandType::Memory,
                        operand_regex: spec.operand_regex.to_string(),
                    });
                }
                let value = token
                    .parse::<u32>()
                    .map_err(|_| SemanticError::ParseInt(token))?;
                let bit_count = spec.bit_count;
                Ok(InstructionField { value, bit_count })
            }
            OperandType::Label => {
                if !re.is_match(&token) {
                    return Err(SemanticError::OperandDoesNotMatch {
                        operand: token,
                        operand_type: OperandType::Label,
                        operand_regex: spec.operand_regex.to_string(),
                    });
                }
                if let Some(location) = self.symtab.get(&token) {
                    Ok(InstructionField {
                        value: *location,
                        bit_count: spec.bit_count,
                    })
                } else {
                    self.tii.entry(token).or_default().push(TiiEntry {
                        instruction_number: self.instruction_counter,
                        operand_number,
                    });
                    Ok(InstructionField {
                        value: 0,
                        bit_count: spec.bit_count,
                    })
                }
            }
        }
    }

    pub fn analyze_statement(
        &mut self,
        statement: Statement,
    ) -> Result<Instruction, SemanticError> {
        let operation_name = statement.operation_name.unwrap();
        let operation = self
            .optspec
            .get_by_operation_name(operation_name.as_str())?;
        let opcode = operation.opcode;

        let expected_operands = operation.operands.clone();

        let operands = if let Some(operands) = statement.operands {
            if operands.len() < expected_operands.len() {
                return Err(SemanticError::TooFewOperands {
                    expected: expected_operands.len(),
                    got: operands.len(),
                });
            } else if operands.len() > expected_operands.len() {
                return Err(SemanticError::TooManyOperands {
                    expected: expected_operands.len(),
                    got: operands.len(),
                });
            } else {
                operands
            }
        } else {
            if expected_operands.len() != 0 {
                return Err(SemanticError::TooFewOperands {
                    expected: expected_operands.len(),
                    got: 0,
                });
            } else {
                vec![]
            }
        };

        let operands: Result<Vec<InstructionField>, SemanticError> = expected_operands
            .iter()
            .zip(operands.iter())
            .enumerate()
            .map(|(i, (spec, token))| {
                let re = Regex::new(spec.operand_regex.as_str())?;
                self.parse_operand(token.clone(), spec, &re, i)
            })
            .collect();

        let operands = operands?;

        let size = (self.optspec.opcode_bit_count
            + operands
                .iter()
                .fold(0, |acc, operand| acc + operand.bit_count)) as u32;

        self.instruction_counter += 1;

        Ok(Instruction {
            opcode: InstructionField {
                value: opcode,
                bit_count: self.optspec.opcode_bit_count,
            },
            operands: Some(operands),
            size,
        })
    }

    pub fn parse(&mut self, statements: Vec<Statement>) -> Result<Vec<Instruction>, SemanticError> {
        let statements = self.normalize(statements)?;
        let mut instructions = Vec::<Instruction>::new();
        for statement in statements {
            if let Some(label) = &statement.label {
                match self.symtab.contains_key(label) {
                    true => {
                        return Err(SemanticError::LabelAlreadyInUse(label.to_string()));
                    }
                    false => {
                        self.symtab.insert(label.to_string(), self.location_counter);

                        // patch

                        let tii_entries = self.tii.get(label).unwrap();

                        for entry in tii_entries {
                            instructions[entry.instruction_number]
                                .operands
                                .as_mut()
                                .unwrap()[entry.operand_number]
                                .value = self.location_counter;
                        }
                    }
                };
            }
            if statement.operation_name.is_some() {
                let instruction = self.analyze_statement(statement)?;
                self.location_counter += instruction.size;
                instructions.push(instruction);
            }
        }
        Ok(instructions)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{instruction::Statement, semantic_parser::SemanticParser};

    #[test]
    fn test() {
        let statement1 = Statement {
            label: Some("MOVE".to_string()),
            operation_name: None,
            operands: None,
        };
        let statement2 = Statement {
            label: None,
            operation_name: Some("MOVER".to_string()),
            operands: Some(vec!["R0".to_string(), "0".to_string()]),
        };
        let statements = vec![statement1, statement2];

        let mut semantic_parser = SemanticParser::new();
        let instructions = semantic_parser.parse(statements).unwrap();
        assert_eq!(instructions.len(), 1);
    }
}
