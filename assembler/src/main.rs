mod assembler;
mod bin_generator;
mod delimiter;
mod instruction;
mod parser;
mod semantic_analyzer;
mod writer;

use crate::assembler::MyAssembler;
use args::Args;
use std::process;

fn main() {
    let args = match Args::parse() {
        Ok(args) => args,
        Err(err) => {
            println!("Failed to parse arguments:\n\t{}", err);
            process::exit(1);
        }
    };
    let input_filename = match args.input_filename.clone() {
        Some(filename) => {
            if filename.split('.').last().unwrap() == "asm" {
                filename
            } else {
                println!("Assembler only accepts .asm files");
                process::exit(1);
            }
        }
        None => {
            println!("Usage: assembler <filename.asm> [--debug] [--pretty] [--log=<console|file>]");
            process::exit(1);
        }
    };
    let mut assembler = match MyAssembler::new(args) {
        Ok(assembler) => assembler,
        Err(err) => {
            println!("Failed to create assembler:\n\t{}", err);
            process::exit(1);
        }
    };
    if let Err(err) = assembler.assemble(&input_filename) {
        println!("Failed to assemble:\n\t{}", err);
        process::exit(1);
    };
}
