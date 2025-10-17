use logger::{Logger, LoggerError};

#[derive(Debug)]
pub struct Instruction {
    pub label: Option<String>,
    pub operation_name: String,
    pub operands: Option<Vec<String>>,
}

impl Instruction {
    pub fn new() -> Self {
        Self {
            label: None,
            operation_name: String::new(),
            operands: None,
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn set_operation_name(&mut self, operation_name: String) {
        self.operation_name = operation_name;
    }

    pub fn add_operand(&mut self, operand: String) {
        if let Some(operands) = &mut self.operands {
            operands.push(operand);
        } else {
            self.operands = Some(vec![operand]);
        }
    }

    pub fn print_instruction(
        instruction: &Instruction,
        opcode: Option<u32>,
        logger: &mut Logger,
    ) -> Result<(), LoggerError> {
        if let Some(opcode) = opcode {
            logger.log(format!(
                "Instruction: Operation_name = {}, Opcode = {}, Operands = {:?}",
                instruction.operation_name, opcode, instruction.operands
            ))?;
        }
        logger.log(format!(
            "Instruction: Operation_name = {}, Operands = {:?}",
            instruction.operation_name, instruction.operands
        ))?;
        Ok(())
    }
}

pub struct InstructionField {
    pub value: u32,
    pub bit_count: u8,
}

pub struct SemanticallyParsedInstruction {
    pub opcode: InstructionField,
    pub operands: Option<Vec<InstructionField>>,
}
