mod writer;
mod instruction;
mod assembler;

use assembler::MyAssembler;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide the path to the assembly file.");
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
    let mut assembler = MyAssembler::new(debug, pretty);
    assembler.assemble(&args[1]);
}
