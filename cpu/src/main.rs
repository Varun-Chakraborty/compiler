mod cpu;
mod handler;
mod instruction;
mod memory;
mod register;

use crate::cpu::MyCPU;
use args::Args;
use std::process;

pub fn main() {
    let args = match Args::parse() {
        Ok(args) => args,
        Err(err) => {
            println!("Failed to parse arguments:\n\t{}", err);
            std::process::exit(1);
        }
    };
    let input_filename = match args.input_filename.clone() {
        Some(filename) => {
            if filename.split('.').last().unwrap() == "bin" {
                filename
            } else {
                println!("CPU only accepts .bin files");
                process::exit(1);
            }
        }
        None => {
            println!("Usage: assembler <filename.bin> [--debug] [--log=<console|file>]");
            process::exit(1);
        }
    };
    let mut cpu = match MyCPU::new(&args) {
        Ok(cpu) => cpu,
        Err(err) => {
            println!("Failed to create CPU:\n\t{}", err);
            std::process::exit(1);
        }
    };
    if args.debug {
        println!("Debug mode enabled.");
    }
    match cpu.load_binary(&input_filename) {
        Ok(()) => {}
        Err(err) => {
            println!("Failed to load binary:\n\t{}", err);
            std::process::exit(1);
        }
    };
    match cpu.run() {
        Ok(()) => {}
        Err(err) => {
            println!("Failed to run:\n\t{}", err);
            std::process::exit(1);
        }
    };
}
