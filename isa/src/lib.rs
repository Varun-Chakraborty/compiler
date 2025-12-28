use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum OperandType {
    Register,
    Memory,
    Label,
    Constant,
}

impl Display for OperandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandType::Register => write!(f, "Register"),
            OperandType::Memory => write!(f, "Memory"),
            OperandType::Label => write!(f, "Label"),
            OperandType::Constant => write!(f, "Constant"),
        }
    }
}

#[derive(Clone)]
pub struct OperandSpec {
    pub operand_type: OperandType,
    pub operand_regex: String,
    pub bit_count: u8,
}

impl OperandSpec {
    fn new(operand_regex: &str, bit_count: u8, operand_type: OperandType) -> Self {
        Self {
            operand_type,
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

pub struct OptSpec {
    pub opcode_bit_count: u8,
    opttab: Vec<Operation>,
}

impl OptSpec {
    pub fn clone() -> Self {
        let reg = OperandSpec::new("^R[0-3]$", 2, OperandType::Register);
        let mem = OperandSpec::new("^[0-9]+$", 4, OperandType::Memory);
        let label = OperandSpec::new("^[A-Z]+$", 8, OperandType::Label);
        let constant = OperandSpec::new("^-?[0-9]+$", 8, OperandType::Constant);

        let no_operands = vec![];
        let reg_mem = vec![reg.clone(), mem.clone()];
        let reg_reg_reg = vec![reg.clone(), reg.clone(), reg.clone()];
        let reg_reg_const = vec![reg.clone(), reg.clone(), constant.clone()];
        let reg_only = vec![reg.clone()];
        let reg_reg = vec![reg.clone(), reg.clone()];
        let label = vec![label.clone()];
        let reg_const = vec![reg.clone(), constant.clone()];
        let constant_only = vec![constant.clone()];

        Self {
            opcode_bit_count: 6,
            opttab: vec![
                Operation::new("HALT", 0, no_operands.clone()),
                Operation::new("MOVER", 1, reg_mem.clone()),
                Operation::new("MOVEI", 2, reg_const.clone()),
                Operation::new("MOVEM", 3, reg_mem.clone()),
                Operation::new("IN", 5, reg_only.clone()),
                Operation::new("OUT", 6, reg_only.clone()),
                Operation::new("OUT_16", 7, no_operands.clone()),
                Operation::new("OUT_CHAR", 4, reg_only.clone()),
                Operation::new("ADD", 8, reg_reg_reg.clone()),
                Operation::new("ADDI", 9, reg_reg_const.clone()),
                Operation::new("ADC", 10, reg_reg_reg.clone()),
                Operation::new("ADCI", 11, reg_reg_const.clone()),
                Operation::new("SUB", 13, reg_reg_reg.clone()),
                Operation::new("SUBI", 14, reg_reg_const.clone()),
                Operation::new("SBC", 15, reg_reg_reg.clone()),
                Operation::new("SBCI", 16, reg_reg_const.clone()),
                Operation::new("MULT", 18, reg_reg_reg.clone()),
                Operation::new("MULTI", 19, reg_reg_const.clone()),
                Operation::new("MULT_16", 20, reg_only.clone()),
                Operation::new("JMP", 21, label.clone()),
                Operation::new("JZ", 22, label.clone()),
                Operation::new("JNZ", 23, label.clone()),
                Operation::new("AND", 24, reg_reg_reg.clone()),
                Operation::new("OR", 25, reg_reg_reg.clone()),
                Operation::new("XOR", 26, reg_reg_reg.clone()),
                Operation::new("NOT", 27, reg_only.clone()),
                Operation::new("SHL", 28, reg_only.clone()),
                Operation::new("PUSH", 32, reg_only.clone()),
                Operation::new("POP", 33, reg_only.clone()),
                Operation::new("CALL", 34, label.clone()),
                Operation::new("RET", 35, no_operands.clone()),
                Operation::new("SHR", 29, reg_only.clone()),
                Operation::new("CMP", 30, reg_reg.clone()),
                Operation::new("CMPI", 31, reg_const.clone()),
                Operation::new("JG", 38, label.clone()),
                Operation::new("JGE", 36, label.clone()),
                Operation::new("JL", 37, label.clone()),
                Operation::new("JLE", 39, label.clone()),
                Operation::new("JNE", 40, label.clone()),
                Operation::new("JE", 41, label.clone()),
                Operation::new("DB", 42, constant_only.clone()),
            ],
        }
    }

    pub fn get_by_opcode(&self, opcode: &u32) -> Option<&Operation> {
        self.opttab.iter().find(|op| op.opcode == *opcode)
    }

    pub fn get_by_operation_name(&self, operation_name: &str) -> Option<&Operation> {
        self.opttab
            .iter()
            .find(|op| op.operation_name == operation_name)
    }

    pub fn contains_opcode(&self, opcode: u32) -> bool {
        return self.opttab.iter().any(|op| op.opcode == opcode);
    }

    pub fn contains_operation_name(&self, operation: &str) -> bool {
        return self.opttab.iter().any(|op| op.operation_name == operation);
    }
}
