use cpu::MyCPU;
use args::Args;
use std::{fs::File, io::{BufReader, Read}, process};

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
            println!("Usage: cpu <filename.bin> [--debug] [--log=<console|file>]");
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
    
    let file = File::open(&input_filename).expect("Failed to open file");
    println!("Loading binary file: {}", input_filename);
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).expect("Failed to read file");
    
    if let Err(err) = cpu.load_binary(buffer) {
        println!("Failed to load binary:\n\t{}", err);
        std::process::exit(1);
    };

    if let Err(err) = cpu.run() {
        println!("Failed to run:\n\t{}", err);
        std::process::exit(1);
    };
}
