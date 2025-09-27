use crate::{
    instruction::SemanticallyParsedInstruction,
    writer::{Writer, WriterError},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinGenError {
    #[error("{0}")]
    WriterError(#[from] WriterError),
}

pub struct BinGenerator {}

impl BinGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_binary(
        &mut self,
        instruction: SemanticallyParsedInstruction,
        writer: &mut Writer,
        location_counter: &mut u32,
    ) -> Result<(), BinGenError> {
        // write the opcode
        let bits = instruction.opcode.bit_count;
        writer.write(instruction.opcode.value, bits)?;
        *location_counter += bits as u32;

        let mut bits = 0 as u8;

        if let Some(operands) = instruction.operands {
            for operand in operands {
                // write it down
                bits += operand.bit_count;
                writer.write(operand.value, operand.bit_count)?;
            }
        }

        *location_counter += bits as u32;

        writer.new_line()?;
        Ok(())
    }
}
