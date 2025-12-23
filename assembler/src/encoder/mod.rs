pub mod delimiter;

use std::mem;

use self::delimiter::DelimiterTable;
use super::parser::instruction::Instruction;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncoderError {
    #[error("{0} can't be converted to a digit")]
    ParseInt(char),
}

pub struct Encoder {
    bits_stream: Vec<u8>,
    location_counter: u32,
    delimiter_table: DelimiterTable,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            bits_stream: Vec::new(),
            location_counter: 0,
            delimiter_table: DelimiterTable::new(),
        }
    }

    pub fn generate_binary_for_instruction(
        &mut self,
        instruction: Instruction,
    ) -> Result<(), EncoderError> {
        let bits = instruction.opcode.bit_count;
        let binary = format!(
            "{:0>width$b}",
            instruction.opcode.value,
            width = bits as usize
        );
        for bit in binary.chars() {
            self.bits_stream
                .push(bit.to_digit(10).ok_or(EncoderError::ParseInt(bit))? as u8);
        }
        self.location_counter += bits as u32;
        self.delimiter_table.append(String::from(' '), self.location_counter as usize);

        if let Some(operands) = instruction.operands {
            for operand in operands {
                self.location_counter += operand.bit_count as u32;
                let binary = format!(
                    "{:0>width$b}",
                    operand.value,
                    width = operand.bit_count as usize
                );
                for bit in binary.chars() {
                    self.bits_stream
                        .push(bit.to_digit(10).ok_or(EncoderError::ParseInt(bit))? as u8);
                }

                self.delimiter_table.append(String::from(", "), self.location_counter as usize);
            }
        }

        self.delimiter_table.delete_last();
        self.delimiter_table.append('\n'.to_string(), self.location_counter as usize);
        Ok(())
    }

    fn pack_bytes(&mut self) -> Vec<u8> {
        let mut result = Vec::new();
        let len = self.bits_stream.len() as u32;
        if len % 8 != 0 {
            let padding = 8 - (len % 8);
            for _ in 0..padding {
                self.bits_stream.push(0);
            }
        }
        for i in 0..self.bits_stream.len() / 8 {
            let mut byte = 0;
            for j in 0..8 {
                byte |= self.bits_stream[i * 8 + j] << (7 - j);
            }
            result.push(byte);
        }
        len.to_be_bytes().iter().for_each(|b| result.push(*b));
        result
    }

    pub fn encode(
        &mut self,
        instructions: Vec<Instruction>
    ) -> Result<(Vec<u8>, DelimiterTable), EncoderError> {
        for instruction in instructions {
            self.generate_binary_for_instruction(instruction)?
        }

        Ok((self.pack_bytes(), mem::take(&mut self.delimiter_table)))
    }
}
