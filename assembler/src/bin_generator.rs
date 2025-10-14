use crate::{
    delimiter::DelimiterTable, instruction::SemanticallyParsedInstruction, writer::{Writer, WriterError}
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
        delimiter_table: &mut DelimiterTable,
    ) -> Result<(), BinGenError> {
        // write the opcode
        let bits = instruction.opcode.bit_count;
        writer.write(instruction.opcode.value, bits)?;
        *location_counter += bits as u32;

        delimiter_table.append(' '.to_string(), *location_counter as usize);

        if let Some(operands) = instruction.operands {
            for operand in operands {
                // write it down
                *location_counter += operand.bit_count as u32;
                writer.write(operand.value, operand.bit_count)?;

                delimiter_table.append(", ".to_string(), *location_counter as usize);
            }
        }

        delimiter_table.delete_last();
        delimiter_table.append('\n'.to_string(), *location_counter as usize);
        Ok(())
    }
}
