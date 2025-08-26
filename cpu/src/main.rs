mod cpu;
mod instruction;
mod memory;
mod register;

use cpu::MyCPU;

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
    cpu.load_binary(&args[1]);
    cpu.run();
    if debug {
        println!("Execution completed.");
        cpu.print_registers();
        cpu.print_program_counter();
    }
    println!("End of Execution.");
}
