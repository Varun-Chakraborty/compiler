use std::error::Error;

pub struct OperandSpec {
    pub operand_regex: &'static str,
    pub bit_count: u32,
}

pub struct Operation {
    pub operation_name: &'static str,
    pub opcode: u32,
    pub operands: &'static [OperandSpec],
}

impl Operation {
    fn new(operation_name: &'static str, opcode: u32, operands: &'static [OperandSpec]) -> Self {
        Self {
            operation_name: operation_name,
            opcode,
            operands,
        }
    }
}

pub struct OptSpec {
    pub opcode_bit_count: u32,
    opttab: Vec<Operation>,
}

impl OptSpec {
    pub fn clone() -> Self {
        const NO_OPERANDS: &[OperandSpec] = &[];
        const REG_MEM: &[OperandSpec] = &[
            OperandSpec {
                operand_regex: "R[0-3]",
                bit_count: 2,
            },
            OperandSpec {
                operand_regex: "[0-9]+",
                bit_count: 4,
            },
        ];
        const REG_REG_MEM: &[OperandSpec] = &[
            OperandSpec {
                operand_regex: "R[0-3]",
                bit_count: 2,
            },
            OperandSpec {
                operand_regex: "R[0-3]",
                bit_count: 2,
            },
            OperandSpec {
                operand_regex: "[0-9]+",
                bit_count: 4,
            },
        ];
        const REG_ONLY: &[OperandSpec] = &[OperandSpec {
            operand_regex: "R[0-3]",
            bit_count: 2,
        }];
        const LABEL: &[OperandSpec] = &[OperandSpec {
            operand_regex: "[a-zA-Z]+",
            bit_count: 8,
        }];
        const MEM_CONSTANT: &[OperandSpec] = &[
            OperandSpec {
                operand_regex: "[0-9]+",
                bit_count: 4,
            },
            OperandSpec {
                operand_regex: "[0-9]+",
                bit_count: 8,
            },
        ];
        Self {
            opcode_bit_count: 4,
            opttab: vec![
                Operation::new("HALT", 0, NO_OPERANDS),
                Operation::new("MOVER", 1, REG_MEM),
                Operation::new("MOVEM", 2, REG_MEM),
                Operation::new("IN", 3, REG_ONLY),
                Operation::new("OUT", 4, REG_ONLY),
                Operation::new("ADD", 5, REG_REG_MEM),
                Operation::new("SUB", 6, REG_REG_MEM),
                Operation::new("MULT", 7, REG_REG_MEM),
                Operation::new("JMP", 8, LABEL),
                Operation::new("JZ", 9, LABEL),
                Operation::new("JNZ", 10, LABEL),
                Operation::new("DC", 11, MEM_CONSTANT),
            ],
        }
    }

    pub fn get_by_opcode(&self, opcode: &u32) -> Result<&Operation, Box<dyn Error>> {
        return match self.opttab.iter().find(|op| op.opcode == *opcode) {
            Some(op) => Ok(op),
            None => Err(format!("Invalid opcode: {}", opcode).into()),
        };
    }

    pub fn get_by_operation_name(&self, operation_name: &str) -> Result<&Operation, Box<dyn Error>> {
        return match self
            .opttab
            .iter()
            .find(|op| op.operation_name == operation_name)
        {
            Some(op) => Ok(op),
            None => Err(format!("Invalid operation name: {}", operation_name).into()),
        };
    }

    pub fn contains_opcode(&self, opcode: u32) -> bool {
        return self.opttab.iter().any(|op| op.opcode == opcode);
    }

    pub fn contains_operation_name(&self, operation: &str) -> bool {
        return self.opttab.iter().any(|op| op.operation_name == operation);
    }
}
