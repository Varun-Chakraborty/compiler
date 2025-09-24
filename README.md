# Compiler - Rust Version
![Rust](https://img.shields.io/badge/Rust-1.89.0-orange)
![MIT](https://img.shields.io/badge/License-MIT-green)
![Version](https://img.shields.io/badge/Version-0.1.0-blue)
[![Release](https://github.com/Varun-Chakraborty/compiler/actions/workflows/release.yml/badge.svg)](https://github.com/Varun-Chakraborty/compiler/actions/workflows/release.yml)

> **Compiler** is a **from-scratch CPU simulator** paired with a **simple assembler** that can translate custom assembly language into binary code.

### Archived Java Version
This project was originally started as a Java implementation to learn the basics of CPU simulation and assembly.  
That version has now been **archived** and preserved in the [`java-archive`](https://github.com/Varun-Chakraborty/compiler/tree/java-archive) tag.  
The active development is now focused on the Rust port, due to its closer alignment with systems programming concepts.

**Note:** You’ll need Rust installed to run these Rust-based tools.

## Overview
- The CPU executes basic instructions like data movement, arithmetic, conditional jumps, input/output, and halting.  
- The assembler converts human-readable assembly into a `.bin` file, which the CPU can then run.

This project is being built to learn **system software** and understand **how CPUs work** at a low level.

---

## Table of Contents
- [Quick Start](#quick-start)
- [Components](#components)
  - [ISA](#isa)
  - [CPU](#cpu)
  - [Assembler](#assembler)
- [How It Works](#how-it-works)
- [Examples](#examples)
- [Verification](#verification)
- [Current Limitations](#current-limitations)
- [Future Improvements](#future-improvements)
- [Motivation](#motivation)

## Quick Start
Example assembly codes are present in the repository in the [`examples`](./examples) folder.

1. Clone the repository: 
    ```
    git clone https://github.com/Varun-Chakraborty/compiler.git
    ```
2. Navigate to the project directory:
    ```
    cd compiler
    ```
3. Run the assembler:
    ```
    cargo run -p assembler examples/fact.asm
    ```
4. Run the CPU:
    ```
    cargo run -p cpu output.bin
    ```

## Components
### ISA
Symbol table mapping for opcodes:
    <table>
        <tr>
            <th>Opcode</th>
            <th>Mnemonic</th>
            <th>Expected Count of Arguments</th>
        </tr>
        <tr>
            <td>0000</td>
            <td>HALT</td>
            <td>0</td>
        </tr>
        <tr>
            <td>0001</td>
            <td>MOVER</td>
            <td>2 (R, M)</td>
        </tr>
        <tr>
            <td>0010</td>
            <td>MOVEM</td>
            <td>2 (R, M)</td>
        </tr>
        <tr>
            <td>0011</td>
            <td>IN</td>
            <td>1 (R)</td>
        </tr>
        <tr>
            <td>0100</td>
            <td>OUT</td>
            <td>1 (R)</td>
        </tr>
        <tr>
            <td>0101</td>
            <td>ADD</td>
            <td>3 (R, R, M) or 2 (R, M)</td>
        </tr>
        <tr>
            <td>0110</td>
            <td>SUB</td>
            <td>3 (R, R, M) or 2 (R, M)</td>
        </tr>
        <tr>
            <td>0111</td>
            <td>MULT</td>
            <td>3 (R, R, M) or 2 (R, M)</td>
        </tr>
        <tr>
            <td>1000</td>
            <td>JMP</td>
            <td>1 (M)</td>
        </tr>
        <tr>
            <td>1001</td>
            <td>JZ</td>
            <td>1 (M)</td>
        </tr>
        <tr>
            <td>1010</td>
            <td>JNZ</td>
            <td>1 (M)</td>
        </tr>
        <tr>
            <td>1011</td>
            <td>DC</td>
            <td>2 (M, V)</td>
        </tr>
    </table>

- **NOTE:** Some instructions that accept 3 operands can also be written with 2. The assembler automatically expands them.

**Operands**
- R: Register
- M: Memory Address [Data Memory or Program Memory (as per the context)]
- V: Constant

For more details, refer to the [isa crate](./isa/src/lib.rs)

### CPU
- Executes a custom instruction set.
- Supports various opcodes as defined in [the ISA](#isa).
- Keeps track of:
    - **Registers** (R0, R1, R2, R3)
    - **Data memory**
    - **Program memory**
    - **Program counter (PC)**

- Supports two modes:
    - **Normal mode**: Executes instructions sequentially.
    - **Debug mode**: Prints detailed execution steps.

### Assembler
(One pass assembler)
- Converts `.asm` source files into `.bin` file of raw binary always and `.txt` files of ASCII `0` and `1` bits in `--debug` mode and `--pretty` mode.
- Instruction format:  
    `[label:] <4-bit opcode> [<2-bit register> <4-bit operand> [<4-bit operand3>] [<8-bit program memory address (in case of labels)>]]`

    - Here, [] are optional and <> are required parts of the instruction.
- Uses Symbol Table to resolve labels.
- Uses Table of Incomplete Instructions to resolve forward references.

- Operand format:
    - **Opcode**: 4 bits (0-15)
    - **Register**: 2 bits (R0 = 00, R1 = 01, R2 = 10, R3 = 11)
    - **Data Memory Address**: 4 bits (0-15)
    - **Program Memory Address**: 8 bits (0-255)

- Supports three modes:
    - **Normal mode**: Converts assembly to binary without debug info.
    - **Debug mode**: Outputs detailed assembly-to-binary conversion steps. (`--debug` flag.)
    - **Pretty Debug mode**: Outputs human-readable assembly code alongside binary. (`--debug --pretty` flags.)
    
    NOTE: pretty flag has to be preceded by debug flag else it will not work.
---

## How It Works
<img width="560" height="200" alt="c81c3311-c1da-4d1e-92e3-f5261516a11b" src="https://github.com/user-attachments/assets/b2ff68ea-197e-4c1d-90fc-007955a14c71" />

1. **Write Assembly**

    Example: `ADD R0, R1, 0`
    - This means: add the value at memory location `0` with value at register `R1` and store the result in register `R0`.

    An example assembly code and is present in this repository as [`index.asm`](./index.asm).

2. **Assemble**

    Run the assembler to convert your `.asm` file into a `.bin` file:
    ```bash
    cargo run -p assembler examples/fact.asm
    ```
    This produces raw binary in output.bin.
    
    Note:
    1. The assembler also generates a `.txt` file with ASCII `0` and `1` bits if run in debug mode.
        ```bash
        cargo run -p assembler examples/fact.asm --debug --pretty
        ```
        This produces a human-readable binary alongside the raw binary in debug.txt.
    2. A python script is present in the root of the repository to verify if the raw binary matches the ASCII representation (generated in debug mode).
        You can run it as:
        ```bash
        python3 convertBinToASCIIBin.py output.bin
        ```
        This will print the ASCII representation of the binary in the console.

3. **Run on CPU**

    Pass output.bin to the CPU simulator:
    ```
    cargo run -p cpu output.bin
    ```
    The CPU will:
    - Load the program into instruction memory.
    - Fetch, decode, and execute each instruction.
    - Print output as per the instructions, asking for input or displaying the value of a register.
## Example
**program.asm**
```
      IN R0               ; Input the number
      MOVEM R0, 1         ; Move input to memory location 1
      MOVER R1, 1         ; Move input value at memory location 1 to R1
      DC 1, 1             ; Constants; declare a constant of value 1 at memory location 1
      MOVER R0, 1         ; Move value at memory location 1 i.e. 1 to R0
LOOP: MOVEM, R1, 0        ; Support of labels; Move input to memory location 0
      MULT R0, 0          ; Multiply value at R0 (default 1 for the first iteration) with input
      SUB R1, 1           ; Subtract 1 (at memory location 1) from input
      JNZ LOOP            ; Jump to loop if input is not 0
      OUT R0              ; Output the result
      HALT                ; END of program

```
As you might have guessed, the above program calculates the factorial of the input number.

**Output (Normal Mode)**
```
Loading binary file: output.bin
Binary file loaded successfully.
Starting execution...
Enter value for register 0: 5
Output from register 0: 120
End of Execution.
```

You can use the `--debug` flag to run the CPU in `debug mode` to visualize the execution of each instruction.
The complete command would be:
```bash
cargo run -p cpu output.bin --debug
```

---

## Verification
The python script [`convertBinToASCIIBin.py`](./convertBinToASCIIBin.py) can be used to verify the binary output by converting it to ASCII `0` and `1` bits.
Run it as follows:
```bash
python3 convertBinToASCIIBin.py output.bin
```
This will print the ASCII representation of the binary to the console, which can be compared with the expected output.

_This step is optional and mainly for debugging or cross-checking the assembler’s output._

## Current Limitations
- Input/Output is basic (manual IN and OUT instructions).

---
## Future Improvements
- Create a REPL for live assembly and execution.
- Support for more registers and larger memory space.
- Support for floating point operations and more instructions.

---
## Motivation
"Feels good to write 0s and 1s and see them do something."

This project is a practical step toward learning system software by building a CPU from scratch, understanding the fetch-decode-execute cycle, and bridging theory with a working implementation.

---
## License
The project is released under the [MIT License](./LICENSE).

---
## Contributing
Contributions are welcome! Please fork the repository and create a pull request.
