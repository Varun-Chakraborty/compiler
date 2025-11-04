use crate::{
    delimiter::DelimiterTable,
    instruction::SemanticallyParsedInstruction,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinGenError {
    #[error("{0} can't be converted to a digit")]
    ParseInt(char),
}

pub struct BinGenerator {
    bits_stream: Vec<u8>,
}

impl BinGenerator {
    pub fn new() -> Self {
        Self {
            bits_stream: Vec::new(),
        }
    }


    pub fn patch(&mut self, location: u32, data: u32, bit_count: u8) -> Result<(), BinGenError> {
        let binary = format!("{:0>width$b}", data, width = bit_count as usize);
        for (i, bit) in binary.chars().enumerate() {
            self.bits_stream[location as usize + i] =
                bit.to_digit(10).ok_or(BinGenError::ParseInt(bit))? as u8;
        }
        Ok(())
    }

    pub fn generate_binary(
        &mut self,
        instruction: SemanticallyParsedInstruction,
        location_counter: &mut u32,
        delimiter_table: &mut DelimiterTable,
    ) -> Result<(), BinGenError> {
        let bits = instruction.opcode.bit_count;
        let binary = format!("{:0>width$b}", instruction.opcode.value, width = bits as usize);
        for bit in binary.chars() {
            self.bits_stream.push(
                bit.to_digit(10).ok_or(BinGenError::ParseInt(bit))? as u8,
            );
        }
        *location_counter += bits as u32;
        delimiter_table.append(String::from(' '), *location_counter as usize);

        if let Some(operands) = instruction.operands {
            for operand in operands {

                *location_counter += operand.bit_count as u32;
                let binary = format!("{:0>width$b}", operand.value, width = operand.bit_count as usize);
                for bit in binary.chars() {
                    self.bits_stream.push(
                        bit.to_digit(10).ok_or(BinGenError::ParseInt(bit))? as u8,
                    );
                }

                delimiter_table.append(String::from(", "), *location_counter as usize);
            }
        }

        delimiter_table.delete_last();
        delimiter_table.append('\n'.to_string(), *location_counter as usize);
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

    pub fn get_binary(&mut self) -> Vec<u8> {
        self.pack_bytes()
    }
}
