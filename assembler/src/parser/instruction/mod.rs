#[derive(Debug, Clone, Default)]
pub struct Statement {
    pub label: Option<String>,
    pub operation_name: Option<String>,
    pub operands: Option<Vec<String>>,
}

impl Statement {
    pub fn new() -> Self {
        Self {
            label: None,
            operation_name: None,
            operands: None,
        }
    }

    pub fn set_label(&mut self, label: String) {
        self.label = Some(label);
    }

    pub fn set_operation_name(&mut self, operation_name: String) {
        self.operation_name = Some(operation_name);
    }

    pub fn add_operand(&mut self, operand: String) {
        if let Some(operands) = &mut self.operands {
            operands.push(operand);
        } else {
            self.operands = Some(vec![operand]);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct InstructionField {
    pub value: u32,
    pub bit_count: u8,
}

#[derive(Debug, Clone, Default)]
pub struct Instruction {
    pub opcode: InstructionField,
    pub operands: Option<Vec<InstructionField>>,
    pub size: u32,
}
