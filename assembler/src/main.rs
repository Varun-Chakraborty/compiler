mod assembler;
mod instruction;
mod writer;

use std::process;

use crate::assembler::MyAssembler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: assembler <filename.asm> [--debug] [--pretty]");
        return;
    }
    let debug = args.len() >= 3 && args[2] == "--debug";
    let pretty = args.len() >= 4 && args[3] == "--pretty";
    if debug {
        println!("Debug mode enabled.");
    }
    if pretty {
        println!("ASCII binary would be prettified.");
    }
    let mut assembler = match MyAssembler::new(debug, pretty) {
        Ok(assembler) => assembler,
        Err(err) => {
            println!("Failed to create assembler:\n\t{}", err);
            process::exit(1);
        }
    };
    if let Err(err) = assembler.assemble(&args[1]) {
        println!("Failed to assemble:\n\t{}", err);
        process::exit(1);
    };
}
