#[derive(Clone)]
pub struct OperandSpec {
    pub operand_regex: String,
    pub bit_count: u8,
}

impl OperandSpec {
    fn new(operand_regex: &str, bit_count: u8) -> Self {
        Self {
            operand_regex: operand_regex.to_string(),
            bit_count: bit_count,
        }
    }
}

pub struct Operation {
    pub operation_name: String,
    pub opcode: u32,
    pub operands: Vec<OperandSpec>,
}

impl Operation {
    fn new(operation_name: &str, opcode: u32, operands: Vec<OperandSpec>) -> Self {
        Self {
            operation_name: operation_name.to_string(),
            opcode,
            operands,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OptSpecError {
    #[error("Invalid opcode: {0}")]
    InvalidCode(u32),
    #[error("Invalid operation name: {0}")]
    InvalidOptName(String),
    #[error("{0}")]
    OperationError(String),
}

pub struct OptSpec {
    pub opcode_bit_count: u8,
    opttab: Vec<Operation>,
}

impl OptSpec {
    pub fn clone() -> Self {
        let no_operands = vec![];
        let reg_mem = vec![OperandSpec::new("R[0-3]", 2), OperandSpec::new("[0-9]+", 4)];
        let reg_reg_mem = vec![
            OperandSpec::new("R[0-3]", 2),
            OperandSpec::new("R[0-3]", 2),
            OperandSpec::new("[0-9]+", 4),
        ];
        let reg_only = vec![OperandSpec::new("R[0-3]", 2)];
        let mem_constant = vec![OperandSpec::new("[0-9]+", 4), OperandSpec::new("[0-9]+", 4)];
        let label = vec![OperandSpec::new("[A-Z]+", 8)];
        Self {
            opcode_bit_count: 4,
            opttab: vec![
                Operation::new("HALT", 0, no_operands),
                Operation::new("MOVER", 1, reg_mem.clone()),
                Operation::new("MOVEM", 2, reg_mem),
                Operation::new("IN", 3, reg_only.clone()),
                Operation::new("OUT", 4, reg_only),
                Operation::new("ADD", 5, reg_reg_mem.clone()),
                Operation::new("SUB", 6, reg_reg_mem.clone()),
                Operation::new("MULT", 7, reg_reg_mem),
                Operation::new("JMP", 8, label.clone()),
                Operation::new("JZ", 9, label.clone()),
                Operation::new("JNZ", 10, label),
                Operation::new("DC", 11, mem_constant),
            ],
        }
    }

    pub fn get_by_opcode(&self, opcode: &u32) -> Result<&Operation, OptSpecError> {
        return match self.opttab.iter().find(|op| op.opcode == *opcode) {
            Some(op) => Ok(op),
            None => Err(OptSpecError::InvalidCode(*opcode)),
        };
    }

    pub fn get_by_operation_name(&self, operation_name: &str) -> Result<&Operation, OptSpecError> {
        return match self
            .opttab
            .iter()
            .find(|op| op.operation_name == operation_name)
        {
            Some(op) => Ok(op),
            None => Err(OptSpecError::InvalidOptName(operation_name.into())),
        };
    }

    pub fn contains_opcode(&self, opcode: u32) -> bool {
        return self.opttab.iter().any(|op| op.opcode == opcode);
    }

    pub fn contains_operation_name(&self, operation: &str) -> bool {
        return self.opttab.iter().any(|op| op.operation_name == operation);
    }
}
