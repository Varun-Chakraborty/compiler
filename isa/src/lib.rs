pub struct OperationSpec {
    pub operation_name: String,
    pub opcode: u32,
    pub expected_arguments: u32,
}

impl OperationSpec {
    fn new(operation: &str, opcode: u32, expected_arguments: u32) -> Self {
        Self {
            operation_name: String::from(operation),
            opcode,
            expected_arguments,
        }
    }
}

pub struct OptTab {
    opttab: Vec<OperationSpec>,
}

impl OptTab {
    pub fn clone() -> Self {
        Self {
            opttab: vec![
                OperationSpec::new("MOVER", 0, 2),
                OperationSpec::new("MOVEM", 1, 2),
                OperationSpec::new("ADD", 2, 3),
                OperationSpec::new("SUB", 3, 3),
                OperationSpec::new("HALT", 4, 0),
                OperationSpec::new("IN", 5, 1),
                OperationSpec::new("OUT", 6, 1),
                OperationSpec::new("JMP", 7, 1),
                OperationSpec::new("JZ", 8, 1),
                OperationSpec::new("JNZ", 9, 1),
                OperationSpec::new("MULT", 10, 3),
            ],
        }
    }

    pub fn get_by_opcode(&self, opcode: &u32) -> &OperationSpec {
        return match self.opttab.iter().find(|op| op.opcode == *opcode) {
            Some(op) => op,
            None => panic!("Invalid opcode: {}", opcode),
        };
    }

    pub fn get_by_operation_name(&self, operation: &str) -> &OperationSpec {
        return match self.opttab.iter().find(|op| op.operation_name == operation) {
            Some(op) => op,
            None => panic!("Invalid operation: {}", operation),
        };
    }

    pub fn contains_opcode(&self, opcode: u32) -> bool {
        return self.opttab.iter().any(|op| op.opcode == opcode);
    }

    pub fn contains_operation_name(&self, operation: &str) -> bool {
        return self.opttab.iter().any(|op| op.operation_name == operation);
    }
}
