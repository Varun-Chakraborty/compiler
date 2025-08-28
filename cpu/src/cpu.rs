use isa::{OptTab};
use std::io::{stdin, stdout, Read, Write};
use crate::register::Register;
use crate::memory::Memory;
use crate::instruction::{Instruction};

pub struct MyCPU {
    program_counter: u32,
    eof: u32,
    program_memory: Memory,
    data_memory: Memory,
    register: Register,
    zero_flag: bool,
    debug: bool,
    opttab: OptTab
}

impl MyCPU {
    pub fn new(debug: bool) -> Self {
        return Self {
            program_counter: 0,
            eof: 0,
            opttab: OptTab::clone(),
            zero_flag: false,
            program_memory: Memory::new(256),
            data_memory: Memory::new(256),
            register: Register::new(4),
            debug
        };
    }

    pub fn mover(&mut self, operands: &[u32]) { // move to register
        let register = operands[0];
        let memory = operands[1];
        self.register.set(register, self.data_memory.get(memory));
    }

    pub fn movem(&mut self, operands: &[u32]) { // move from register
        let register = operands[0];
        let memory = operands[1];
        let value = self.register.get(register);
        self.zero_flag = value == 0;
        self.data_memory.set(memory, value);
    }

    pub fn add(&mut self, operands: &[u32]) {
        let dest = operands[0];
        let source = operands[1];
        let memory = operands[2];
        let sum = self.register.get(source) + self.data_memory.get(memory);
        self.zero_flag = sum == 0;
        self.register.set(dest, sum);
    }

    pub fn sub(&mut self, operands: &[u32]) {
        let dest = operands[0];
        let source = operands[1];
        let memory = operands[2];
        let diff = self.register.get(source) - self.data_memory.get(memory);
        self.zero_flag = diff == 0;
        self.register.set(dest, diff);
    }

    pub fn halt(&mut self, _: &[u32]) {
        self.program_counter = self.eof;
    }

    pub fn input(&mut self, operands: &[u32]) {
        let register = operands[0];
        let mut input = String::new();
        print!("Enter value for register {register}: ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().parse().unwrap();
        self.zero_flag = input == 0;
        self.register.set(register, input);
    }

    pub fn output(&mut self, operands: &[u32]) {
        let register = operands[0];
        let value = self.register.get(register);
        self.zero_flag = value == 0;
        println!("Output from register {register}: {value}");
        stdout().flush().expect("Failed to flush stdout");
    }

    pub fn jmp(&mut self, operands: &[u32]) {
        let address = operands[0];
        self.program_counter = address;
    }

    pub fn jz(&mut self, operands: &[u32]) {
        let address = operands[0];
        if self.zero_flag {
            self.program_counter = address;
        }
    }

    pub fn jnz(&mut self, operands: &[u32]) {
        let address = operands[0];
        if !self.zero_flag {
            self.program_counter = address;
        }
    }

    pub fn mult(&mut self, operands: &[u32]) {
        let dest = operands[0];
        let source = operands[1];
        let memory = operands[2];
        let product = self.register.get(source) * self.data_memory.get(memory);
        self.zero_flag = product == 0;
        self.register.set(dest, product);
    }

    pub fn opcodes(&mut self, instruction: Instruction) {
        let opcode = instruction.get_opcode();
        let operands = instruction.get_operands();
        if self.debug {
            println!("Executing instruction at PC {}: Opcode = {}, Operands = {:?}", self.program_counter, opcode, operands);
        }
        self.program_counter = instruction.get_program_counter();
        let operation_name = &self.opttab.get_by_opcode(&opcode).operation_name;
        match operation_name.to_lowercase().as_str() {
            "mover" => self.mover(operands),
            "movem" => self.movem(operands),
            "add" => self.add(operands),
            "sub" => self.sub(operands),
            "halt" => self.halt(operands),
            "in" => self.input(operands),
            "out" => self.output(operands),
            "jmp" => self.jmp(operands),
            "jz" => self.jz(operands),
            "jnz" => self.jnz(operands),
            "mult" => self.mult(operands),
            _ => println!("Invalid opcode: {}", opcode)
        }
    }

    pub fn load_binary(&mut self, filepath: &str) {
        use std::fs::File;
        println!("Loading binary file: {}", filepath);
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
        println!("Binary file loaded successfully. Starting execution...");

    }

    pub fn run(&mut self) {
        while  self.program_counter < self.program_memory.size() && self.program_counter < self.eof {
            let instruction = Instruction::new(&self.program_memory, &mut self.program_counter);
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