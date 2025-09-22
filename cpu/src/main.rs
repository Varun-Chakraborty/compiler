mod cpu;
mod instruction;
mod memory;
mod register;

use crate::cpu::MyCPU;

pub fn main() {
    // read arguments from command line
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cpu <filename.bin> [--debug]");
        std::process::exit(1);
    }
    let debug = args.len() == 3 && args[2] == "--debug";
    if debug {
        println!("Debug mode enabled.");
    }
    let mut cpu = MyCPU::new(debug);
    match cpu.load_binary(&args[1]) {
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
    if debug {
        match cpu.print_registers() {
            Ok(()) => {}
            Err(err) => {
                println!("Failed to print registers:\n\t{}", err);
                std::process::exit(1);
            }
        };
        cpu.print_program_counter();
    }
}
