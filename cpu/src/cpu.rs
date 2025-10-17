use crate::instruction::{Instruction, InstructionError};
use crate::memory::{Memory, MemoryError};
use crate::register::{Register, RegisterError};
use args::Args;
use isa::{OptSpec, OptSpecError};
use logger::{LogTo, Logger, LoggerError};
use std::io::{self, Read, Write, stdin, stdout};
use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum CPUError {
    #[error("{0}")]
    Memory(#[from] MemoryError),
    #[error("{0}")]
    Register(#[from] RegisterError),
    #[error("{0}")]
    IO(#[from] io::Error),
    #[error("{0}")]
    ParseInt(#[from] ParseIntError),
    #[error("{0}")]
    OptSpec(#[from] OptSpecError),
    #[error("Operation {0} not implemented yet")]
    NoImplementation(String),
    #[error("{0}")]
    Instruction(#[from] InstructionError),
    #[error("Logger error: {0}")]
    Logger(#[from] LoggerError),
}

pub struct MyCPU {
    program_counter: u32,
    eof: u32,
    program_memory: Memory,
    data_memory: Memory,
    register: Register,
    zero_flag: bool,
    debug: bool,
    opt_spec: OptSpec,
    logger: Logger,
}

impl MyCPU {
    pub fn new(args: Args) -> Result<Self, CPUError> {
        Ok(Self {
            program_counter: 0,
            eof: 0,
            opt_spec: OptSpec::clone(),
            zero_flag: false,
            program_memory: Memory::new(256),
            data_memory: Memory::new(256),
            register: Register::new(4),
            logger: Logger::new(
                if let Some(filename) = args.filename {
                    filename
                } else {
                    "cpu.txt".to_string()
                },
                if let Some(path) = args.path {
                    path
                } else {
                    "./logs/".to_string()
                },
                if let Some(log_to) = args.log_to {
                    if log_to == "file" {
                        LogTo::File
                    } else {
                        LogTo::Console
                    }
                } else {
                    LogTo::Console
                },
            )?,
            debug: args.debug,
        })
    }

    pub fn mover(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        // move to register
        let register = operands[0];
        let memory = operands[1];
        let value = self.data_memory.get(memory)?;
        self.zero_flag = value == 0;
        self.register.set(register, value)?;
        Ok(())
    }

    pub fn movem(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        // move from register
        let register = operands[0];
        let memory = operands[1];
        let value = self.register.get(register)?;
        self.zero_flag = value == 0;
        self.data_memory.set(memory, value)?;
        Ok(())
    }

    pub fn add(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let source = operands[1];
        let memory = operands[2];
        let sum = self.register.get(source)? + self.data_memory.get(memory)?;
        self.zero_flag = sum == 0;
        self.register.set(dest, sum)?;
        Ok(())
    }

    pub fn sub(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let source = operands[1];
        let memory = operands[2];
        let diff = self.register.get(source)? - self.data_memory.get(memory)?;
        self.zero_flag = diff == 0;
        self.register.set(dest, diff)?;
        Ok(())
    }

    pub fn halt(&mut self, _: &[u32]) {
        self.program_counter = self.eof;
    }

    pub fn input(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let register = operands[0];
        let mut input = String::new();
        print!("Enter value for register {register}: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let input = input.trim().parse()?;
        self.zero_flag = input == 0;
        self.register.set(register, input)?;
        Ok(())
    }

    pub fn output(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let register = operands[0];
        let value = self.register.get(register)?;
        self.zero_flag = value == 0;
        println!("Output from register {register}: {value}");
        stdout().flush()?;
        Ok(())
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

    pub fn mult(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        let dest = operands[0];
        let source = operands[1];
        let memory = operands[2];
        let product = self.register.get(source)? * self.data_memory.get(memory)?;
        self.zero_flag = product == 0;
        self.register.set(dest, product)?;
        Ok(())
    }

    pub fn dc(&mut self, operands: &[u32]) -> Result<(), CPUError> {
        self.data_memory.set(operands[0], operands[1] as u8)?;
        self.eof += 1;
        self.zero_flag = operands[1] == 0;
        Ok(())
    }

    pub fn execute(
        &mut self,
        instruction: Instruction,
        program_counter: u32,
    ) -> Result<(), CPUError> {
        let opcode = instruction.get_opcode();
        let operands = instruction.get_operands();
        if self.debug {
            self.logger.log(format!(
                "Executing instruction at PC {}: Opcode = {}, Operands = {:?}",
                program_counter, opcode, operands
            ))?;
        }
        let operation_name = &self.opt_spec.get_by_opcode(&opcode)?.operation_name;
        match operation_name.to_lowercase().as_str() {
            "mover" => Ok(self.mover(operands)?),
            "movem" => Ok(self.movem(operands)?),
            "add" => Ok(self.add(operands)?),
            "sub" => Ok(self.sub(operands)?),
            "halt" => Ok(self.halt(operands)),
            "in" => Ok(self.input(operands)?),
            "out" => Ok(self.output(operands)?),
            "jmp" => Ok(self.jmp(operands)),
            "jz" => Ok(self.jz(operands)),
            "jnz" => Ok(self.jnz(operands)),
            "mult" => Ok(self.mult(operands)?),
            "dc" => Ok(self.dc(operands)?),
            _ => Err(CPUError::NoImplementation(operation_name.to_string())),
        }
    }

    pub fn load_binary(&mut self, filepath: &str) -> Result<(), CPUError> {
        use std::fs::File;
        println!("Loading binary file: {}", filepath);
        let mut file = File::open(filepath)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        if let Some((&eof_byte, instructions)) = buffer.split_last() {
            for &byte in instructions {
                self.program_memory.set(self.program_counter, byte)?;
                self.program_counter += 1;
            }
            self.eof = eof_byte as u32;
        }

        self.program_counter = 0;
        println!("Binary file loaded successfully.");
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), CPUError> {
        println!("Starting execution...");
        while self.program_counter < self.program_memory.size() && self.program_counter < self.eof {
            let pc = self.program_counter;
            let instruction = Instruction::new(
                &self.program_memory,
                &mut self.program_counter,
                &self.opt_spec,
            )?;
            self.execute(instruction, pc)?;
        }
        println!("End of Execution.");
        if self.debug {
            match self.print_registers() {
                Ok(()) => {}
                Err(err) => {
                    println!("Failed to print registers:\n\t{}", err);
                    std::process::exit(1);
                }
            };
            self.print_program_counter();
        }
        Ok(())
    }

    pub fn print_registers(&self) -> Result<(), CPUError> {
        for i in 0..4 {
            println!("Register {i}: {}", self.register.get(i)?);
        }
        Ok(())
    }

    pub fn print_program_counter(&self) {
        println!("Program Counter: {}", self.program_counter);
    }
}
