use args::Args;
use std::{
    fs::File,
    io::{BufReader, Read},
    process,
};
use vm::MyVM;

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
                println!("VM only accepts .bin files");
                process::exit(1);
            }
        }
        None => {
            println!("Usage: vm <filename.bin> [--debug] [--log=<console|file>]");
            process::exit(1);
        }
    };
    let mut vm = match MyVM::new(&args) {
        Ok(vm) => vm,
        Err(err) => {
            println!("Failed to create VM:\n\t{}", err);
            std::process::exit(1);
        }
    };
    if args.debug {
        println!("Debug mode enabled.");
    }

    let file = match File::open(&input_filename) {
        Ok(file) => file,
        Err(err) => {
            println!("Failed to open file:\n\t{}", err);
            std::process::exit(1);
        }
    };
    println!("Loading binary file: {}", input_filename);
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    match reader.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to read file:\n\t{}", err);
            std::process::exit(1);
        }
    };

    if let Err(err) = vm.load_binary(buffer) {
        println!("Failed to load binary:\n\t{}", err);
        std::process::exit(1);
    };

    if let Err(err) = vm.run() {
        println!("Failed to run:\n\t{}", err);
        std::process::exit(1);
    };
}
