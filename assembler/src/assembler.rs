use std::{fs::File, io::{Read}};
use crate::instruction::Instruction;

pub struct MyAssembler {
    instructions: Instruction,
}

impl MyAssembler {
    pub fn new(debug: bool, pretty: bool) -> Self {
        return Self {
            instructions: Instruction::new(debug, pretty)
        };
    }

    pub fn assemble(&mut self, file_name: &str) {
        use std::io::{BufReader};
        let file = File::open(file_name).expect("Failed to open file");
        let mut buffer = String::new();
        let mut reader = BufReader::new(file);
        let mut token = String::new();
        let mut is_comment = false;
        println!("Assembly file: {}", file_name);
        match reader.read_to_string(&mut buffer) {
            Ok(_) => (),
            Err(e) => panic!("Failed to read file: {}", e)
        }
        for c in buffer.chars() {
            match c {
                ',' | ' ' | '\t' => {
                    if is_comment {
                        continue;
                    }
                    self.instructions.add_token(token);
                },
                ';' | '\n' => {
                    if !is_comment {
                        self.instructions.add_token(token);
                        self.instructions.done();
                    }
                    if c == ';' {
                        is_comment = true;
                    } else {
                        is_comment = false;
                    }
                },
                _ => {
                    if is_comment {
                        continue;
                    }
                    token.push(c);
                    continue;
                }
            }
            token = String::new();
        }
        self.instructions.close();
        self.instructions.print_symtab();
        println!("Assembly completed.");
    }
}