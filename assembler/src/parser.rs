use crate::instruction::Instruction;

#[derive(Debug, thiserror::Error)]
pub enum ParserError {
    #[error("Parsing error: {message}")]
    ParseError { message: String },
    #[error("Opcode is missing at line: {line}")]
    OpcodeMissing { line: String },
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn parse(&mut self, line: &str) -> Result<Instruction, ParserError> {
        let instruction = line.split(';').next().ok_or(ParserError::ParseError {
            message: format!(
                "Unable to parse instruction out of line: {}",
                line.to_string()
            ),
        })?;

        let mut parsed_instruction = Instruction::new();

        let instruction = if let Some((label, instruction)) = instruction.split_once(':') {
            // if instruction.trim().is_empty() {
            //     return Err(ParserError::ParseError {
            //         message: format!("The label cannot be empty: {}", line.to_string()),
            //     });
            // }
            parsed_instruction.set_label(label.trim().to_string());
            instruction
        } else {
            instruction
        }
        .trim();

        if instruction.is_empty() {
            return Ok(parsed_instruction);
        }

        if !instruction.contains(' ') {
            parsed_instruction.set_operation_name(instruction.to_string());
            return Ok(parsed_instruction);
        }

        let (opcode, operands) = instruction
            .split_once(' ')
            .map(|(s, t)| (s.trim(), t.trim()))
            .ok_or(ParserError::ParseError {
                message: format!(
                    "Unable to parse opcode and operands out of line: {}",
                    line.to_string()
                ),
            })?;
        if opcode.ends_with(',') || operands.starts_with(',') {
            return Err(ParserError::OpcodeMissing {
                line: line.to_string(),
            });
        }
        parsed_instruction.set_operation_name(opcode.to_string());
        operands.split(',').try_for_each(|operand| {
            parsed_instruction.add_operand(operand.trim().to_string());
            Ok::<(), ParserError>(())
        })?;
        Ok(parsed_instruction)
    }
}
