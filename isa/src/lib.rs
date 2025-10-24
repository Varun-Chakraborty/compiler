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
    #[error("Invalid operation name: '{0}'")]
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
        let reg = OperandSpec::new("R[0-3]", 2);
        let mem = OperandSpec::new("[0-9]+", 4);
        let label = OperandSpec::new("[A-Z]+", 8);
        let constant = OperandSpec::new("[0-9]+", 8);

        let no_operands = vec![];
        let reg_mem = vec![reg.clone(), mem.clone()];
        let reg_reg_mem = vec![reg.clone(), reg.clone(), mem.clone()];
        let reg_reg_const = vec![reg.clone(), reg.clone(), constant.clone()];
        let reg_only = vec![reg.clone()];
        let reg_reg = vec![reg.clone(), reg.clone()];
        let mem_constant = vec![mem.clone(), constant.clone()];
        let label = vec![label.clone()];
        let reg_const = vec![reg.clone(), constant.clone()];

        Self {
            opcode_bit_count: 6,
            opttab: vec![
                Operation::new("HALT", 0, no_operands.clone()),
                Operation::new("MOVER", 1, reg_mem.clone()),
                Operation::new("MOVERI", 2, reg_const.clone()),
                Operation::new("MOVEM", 3, reg_mem),
                Operation::new("MOVEMI", 4, mem_constant),
                Operation::new("IN", 5, reg_only.clone()),
                Operation::new("OUT", 6, reg_only.clone()),
                Operation::new("ADD", 7, reg_reg_mem.clone()),
                Operation::new("ADDI", 8, reg_reg_const.clone()),
                Operation::new("SUB", 9, reg_reg_mem.clone()),
                Operation::new("SUBI", 10, reg_reg_const.clone()),
                Operation::new("MULT", 11, reg_reg_mem.clone()),
                Operation::new("MULTI", 12, reg_reg_const.clone()),
                Operation::new("DIV", 13, reg_reg_mem.clone()),
                Operation::new("DIVI", 14, reg_reg_const.clone()),
                Operation::new("MOD", 15, reg_reg_mem.clone()),
                Operation::new("MODI", 16, reg_reg_const.clone()),
                Operation::new("JMP", 17, label.clone()),
                Operation::new("JZ", 18, label.clone()),
                Operation::new("JNZ", 19, label.clone()),
                Operation::new("AND", 20, reg_reg_mem.clone()),
                Operation::new("OR", 21, reg_reg_mem.clone()),
                Operation::new("XOR", 22, reg_reg_mem.clone()),
                Operation::new("NOT", 23, reg_only.clone()),
                Operation::new("SHL", 24, reg_only.clone()),
                Operation::new("SHR", 25, reg_only.clone()),
                Operation::new("CMP", 26, reg_reg),
                Operation::new("CMPI", 27, reg_const),
                Operation::new("PUSH", 28, reg_only.clone()),
                Operation::new("POP", 29, reg_only),
                Operation::new("CALL", 30, label.clone()),
                Operation::new("RET", 31, no_operands),
                Operation::new("JE", 32, label.clone()),
                Operation::new("JNE", 33, label.clone()),
                Operation::new("JG", 34, label.clone()),
                Operation::new("JL", 35, label),
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
