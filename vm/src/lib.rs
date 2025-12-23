mod handler;
mod instruction;
mod memory;
mod register;

use crate::instruction::{Instruction, InstructionError};
use crate::memory::{Memory, MemoryError};
use crate::register::{Register, RegisterError};
use args::Args;
use isa::{OptSpec, OptSpecError};
use logger::{LogTo, Logger, LoggerError};
use std::io;
use std::num::ParseIntError;

#[derive(Debug, thiserror::Error)]
pub enum VMError {
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
    #[error("Invalid binary")]
    InvalidBinary,
    #[error("Error converting Vec to slice")]
    VecToSlice,
}

#[derive(Debug, Copy, Clone)]
pub struct Flags {
    pub zero: bool,
    pub sign: bool,
    pub overflow: bool,
    pub carry: bool,
}

pub struct MyVM {
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

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Read,
    Write,
}

#[derive(Debug, Clone)]
pub struct MemoryAccess {
    pub address: u32,
    pub value: u8,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub instruction_str: String,
    pub address: u32,
    pub changed_flags: Vec<String>,
    pub changed_regs: Vec<String>,
    pub memory_access: Option<MemoryAccess>,
    pub is_halted: bool,
    pub stack_pointer: u32,
}

#[derive(Clone)]
pub struct VMState {
    pub program_counter: u32,
    pub registers: Register<u8>,
    pub flags: Flags,
    pub program_memory: Memory<u8>,
    pub data_memory: Memory<u8>,
    pub stack_pointer: u32,
}

impl MyVM {
    pub fn new(args: &Args) -> Result<Self, VMError> {
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
                    String::from("vm.txt")
                },
                args.path.clone(),
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
    ) -> Result<ExecutionStep, VMError> {
        let opcode = instruction.get_opcode();
        let operands = instruction.get_operands();
        if self.debug {
            self.logger.log(format!(
                "Executing instruction at PC {}: Opcode = {}, Operands = {:?}",
                program_counter, opcode, operands
            ))?;
        }
        let operation_name = &self.opt_spec.get_by_opcode(&opcode)?.operation_name;

        let changes = match operation_name.to_lowercase().as_str() {
            "halt" => Ok(self.halt(operands)?),
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
            "jmp" => Ok(self.jmp(operands)?),
            "jz" => Ok(self.jz(operands)?),
            "jnz" => Ok(self.jnz(operands)?),
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
            "jge" => Ok(self.jge(operands)?),
            "jl" => Ok(self.jl(operands)?),
            _ => Err(VMError::NoImplementation(operation_name.to_string())),
        }?;

        Ok(ExecutionStep {
            instruction_str: format!("{:?}", instruction),
            address: program_counter,
            changed_flags: changes.flags,
            changed_regs: changes.registers,
            memory_access: changes.memory_access,
            is_halted: self.eof == self.program_counter,
            stack_pointer: self.stack_pointer,
        })
    }

    pub fn load_binary(&mut self, mut binary_bytes: Vec<u8>) -> Result<(), VMError> {
        self.reset();

        let eof = binary_bytes.split_off(binary_bytes.len() - 4);
        self.eof = u32::from_be_bytes(eof.try_into().map_err(|_| VMError::VecToSlice)?);

        for byte in binary_bytes {
            self.program_memory.set(self.program_counter, byte)?;
            self.program_counter += 1;
        }

        self.program_counter = 0;
        Ok(())
    }

    pub fn step(&mut self) -> Result<ExecutionStep, VMError> {
        let instruction = Instruction::new(
            &self.program_memory,
            &mut self.program_counter,
            &self.opt_spec,
        )?;
        self.execute(instruction, self.program_counter)
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        println!("Starting execution...");
        while self.program_counter < self.program_memory.size() && self.program_counter < self.eof {
            match self.step() {
                Ok(step_info) => {
                    if self.debug {
                        println!("{:?}", step_info);
                    }
                }
                Err(err) => {
                    println!("Failed to execute instruction:\n\t{}", err);
                    std::process::exit(1);
                }
            }
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

    pub fn reset(&mut self) {
        self.program_counter = 0;
        self.flags.zero = false;
        self.flags.carry = false;
        self.flags.sign = false;
        self.flags.overflow = false;
        self.register = Register::new(4);
        self.data_memory = Memory::new(256);
        self.program_memory = Memory::new(256);
    }

    pub fn get_state_struct(&self) -> VMState {
        VMState {
            program_counter: self.program_counter,
            flags: self.flags,
            registers: self.register.clone(),
            data_memory: self.data_memory.clone(),
            program_memory: self.program_memory.clone(),
            stack_pointer: self.stack_pointer,
        }
    }

    pub fn print_registers(&self) -> Result<(), VMError> {
        for i in 0..4 {
            println!("Register {i}: {}", self.register.get(i)?);
        }
        Ok(())
    }

    pub fn print_program_counter(&self) {
        println!("Program Counter: {}", self.program_counter);
    }
}
