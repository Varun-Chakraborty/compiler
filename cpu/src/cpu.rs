use std::{io::{stdin, Read}};
use crate::register::Register;
use crate::memory::Memory;
use crate::instruction::Instruction;

pub struct MyCPU {
    program_counter: u32,
    eof: u32,
    program_memory: Memory,
    data_memory: Memory,
    register: Register,
    debug: bool
}

impl MyCPU {
    pub fn new(debug: bool) -> Self {
        return Self {
            program_counter: 0,
            eof: 0,
            program_memory: Memory::new(256),
            data_memory: Memory::new(256),
            register: Register::new(4),
            debug
        };
    }

    pub fn mover(&mut self, register: u32, memory: u32) { // move to register
        self.register.set(register, self.data_memory.get(memory));
    }

    pub fn movem(&mut self, memory: u32, register: u32) { // move from register
        self.data_memory.set(memory, self.register.get(register));
    }

    pub fn add(&mut self, register1: u32, register2: u32) {
        self.register.set(register1, self.register.get(register1) + self.register.get(register2));
    }

    pub fn sub(&mut self, register1: u32, register2: u32) {
        self.register.set(register1, self.register.get(register1) - self.register.get(register2));
    }

    pub fn halt(&mut self) {
        self.program_counter = self.eof;
    }

    pub fn input(&mut self, register: u32) {
        let mut input = String::new();
        println!("Enter value for register {register}: ");
        stdin().read_line(&mut input).expect("Failed to read line");
        self.register.set(register, input.trim().parse().unwrap());
    }

    pub fn output(&mut self, register: u32) {
        println!("Output from register {register}: {}", self.register.get(register));
    }

    pub fn opcodes(&mut self, instruction: Instruction) {
        let opcode = instruction.get_opcode();
        let operands = instruction.get_operands();
        match opcode {
            0 => self.mover(operands[0], operands[1]),
            1 => self.movem(operands[0], operands[1]),
            2 => self.add(operands[0], operands[1]),
            3 => self.sub(operands[0], operands[1]),
            4 => self.halt(),
            5 => self.input(operands[0]),
            6 => self.output(operands[0]),
            _ => panic!("Invalid opcode")
        }
    }

    pub fn load_binary(&mut self, filepath: &str) {
        use std::fs::File;
        println!("Loading binary file: {}", filepath);
        // check which directory we are in
        println!("Current directory: {}", std::env::current_dir().unwrap().display());
        let mut file = File::open(filepath).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");
    
        if let Some((&eof_byte, instructions)) = buffer.split_last() {
            for byte in instructions {
                for i in 0..8 {
                    let bit = byte >> (7 - i) & 1;
                    self.program_memory.set(self.program_counter, bit);
                    self.program_counter += 1;
                }
            }
            self.eof = eof_byte as u32;
        }

        self.program_counter = 0;

        if self.debug {
            println!("Program Memory: ");
            for i in 0..self.program_memory.size() {
                print!("{} ", self.program_memory.get(i));
                if i % 8 == 7 {
                    println!();
                }
            }
            println!();
        }
        println!("Binary file loaded successfully. Starting execution...");

    }

    pub fn run(&mut self) {
        while  self.program_counter < self.program_memory.size() && self.program_counter < self.eof {
            let instruction = Instruction::new(&self.program_memory, &mut self.program_counter);
            let opcode = instruction.get_opcode();
            let operands = instruction.get_operands();
            if self.debug {
                println!("Executing instruction at PC {}: Opcode = {}, Operands = {:?}", self.program_counter, opcode, operands);
            }
            self.program_counter = instruction.get_program_counter();
            self.opcodes(instruction);
        }
    }

    pub fn print_registers(&self) {
        for i in 0..4 {
            println!("Register {i}: {}", self.register.get(i));
        }
    }

    pub fn print_program_counter(&self) {
        println!("Program Counter: {}", self.program_counter);
    }
}