use crate::instruction::{Instruction, InstructionError};
use crate::memory::{Memory, MemoryError};
use crate::register::{Register, RegisterError};
use args::Args;
use isa::{OptSpec, OptSpecError};
use logger::{LogTo, Logger, LoggerError};
use std::io::{self, Read};
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

pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub overflow: bool,
    pub carry: bool,
}

pub struct MyCPU {
    pub program_counter: u32,
    pub eof: u32,
    pub program_memory: Memory<u8>,
    pub data_memory: Memory<u8>,
    pub register: Register<u8>,
    pub flags: Flags,
    pub debug: bool,
    pub opt_spec: OptSpec,
    pub logger: Logger,
    pub stack_pointer: u32,
}

impl MyCPU {
    pub fn new(args: &Args) -> Result<Self, CPUError> {
        Ok(Self {
            program_counter: 0,
            eof: 0,
            opt_spec: OptSpec::clone(),
            flags: Flags {
                zero: false,
                sign: false,
                overflow: false,
                carry: false,
            },
            program_memory: Memory::new(256),
            data_memory: Memory::new(256),
            register: Register::new(4),
            stack_pointer: 256,
            logger: Logger::new(
                if let Some(filename) = args.filename.clone() {
                    filename
                } else {
                    "cpu.txt".to_string()
                },
                if let Some(path) = args.path.clone() {
                    path
                } else {
                    "./logs/".to_string()
                },
                if let Some(log_to) = args.log_to.clone() {
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

        // implementation of each operation are present in ./handler.rs
        match operation_name.to_lowercase().as_str() {
            "halt" => Ok(self.halt(operands)),
            "in" => Ok(self.input(operands)?),
            "out" => Ok(self.output(operands)?),
            "out_16" => Ok(self.output_16(operands)?),
            "out_char" => Ok(self.output_char(operands)?),
            "mover" => Ok(self.mover(operands, false)?),
            "movei" => Ok(self.mover(operands, true)?),
            "movem" => Ok(self.movem(operands)?),
            "add" => Ok(self.add(operands, false)?),
            "addi" => Ok(self.add(operands, true)?),
            "adc" => Ok(self.adc(operands, false)?),
            "adci" => Ok(self.adc(operands, true)?),
            "sub" => Ok(self.sub(operands, false)?),
            "subi" => Ok(self.sub(operands, true)?),
            "sbc" => Ok(self.sbc(operands, false)?),
            "sbci" => Ok(self.sbc(operands, true)?),
            "mult" => Ok(self.mult(operands, false)?),
            "mult_16" => Ok(self.mult_16(operands, false)?),
            "multi" => Ok(self.mult(operands, true)?),
            "mult_16i" => Ok(self.mult_16(operands, true)?),
            "jmp" => Ok(self.jmp(operands)),
            "jz" => Ok(self.jz(operands)),
            "jnz" => Ok(self.jnz(operands)),
            "and" => Ok(self.and(operands)?),
            "or" => Ok(self.or(operands)?),
            "xor" => Ok(self.xor(operands)?),
            "not" => Ok(self.not(operands)?),
            "shl" => Ok(self.shl(operands)?),
            "shr" => Ok(self.shr(operands)?),
            "cmp" => Ok(self.cmp(operands, false)?),
            "cmpi" => Ok(self.cmp(operands, true)?),
            "push" => Ok(self.push(operands)?),
            "pop" => Ok(self.pop(operands)?),
            "call" => Ok(self.call(operands)?),
            "ret" => Ok(self.ret(operands)?),
            "jge" => Ok(self.jge(operands)),
            "jl" => Ok(self.jl(operands)),
            _ => Err(CPUError::NoImplementation(operation_name.to_string())),
        }
    }

    pub fn load_binary(&mut self, filepath: &str) -> Result<(), CPUError> {
        use std::fs::File;
        println!("Loading binary file: {}", filepath);
        let mut file = File::open(filepath)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let eof = buffer.split_off(buffer.len() - 4);
        self.eof = u32::from_be_bytes(eof.try_into().unwrap());

        let instructions = buffer;
        for byte in instructions {
            self.program_memory.set(self.program_counter, byte)?;
            self.program_counter += 1;
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
