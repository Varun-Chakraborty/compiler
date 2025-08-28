# Compiler - Rust Version

### Archived Java Version
This project was originally started as a Java implementation to learn the basics of CPU simulation and assembly.  
That version has now been **archived** and preserved in the [`java-archive`](https://github.com/Varun-Chakraborty/compiler/tree/java-archive) tag.  
The active development is now focused on the Rust port, due to its closer alignment with systems programming concepts.

**Note:** You’ll need Rust installed to run these Rust-based tools.

## Overview
This project is a **from-scratch CPU simulator** paired with a simple **assembler** that can translate custom assembly language into machine code (represented as ASCII 0/1 bits).

The CPU executes basic instructions like data movement, arithmetic, and halting.  
The assembler converts human-readable assembly into a `.bin` file, which the CPU can then run.

This project is being built to learn **system software** and understand **how CPUs work** at a low level.

---

## Features
# ISA
Symbol table mapping for opcodes:
    <table>
        <tr>
            <th>Opcode</th>
            <th>Mnemonic</th>
            <th>Expected Count of Arguments</th>
        </tr>
        <tr>
            <td>0000</td>
            <td>MOVER</td>
            <td>2 (R, M)</td>
        </tr>
        <tr>
            <td>0001</td>
            <td>MOVEM</td>
            <td>2 (R, M)</td>
        </tr>
        <tr>
            <td>0010</td>
            <td>ADD</td>
            <td>3 (R, R, M)</td>
        </tr>
        <tr>
            <td>0011</td>
            <td>SUB</td>
            <td>3 (R, R, M)</td>
        </tr>
        <tr>
            <td>0100</td>
            <td>HALT</td>
            <td>0</td>
        </tr>
        <tr>
            <td>0101</td>
            <td>IN</td>
            <td>1 (R)</td>
        </tr>
        <tr>
            <td>0110</td>
            <td>OUT</td>
            <td>1 (R)</td>
        </tr>
        <tr>
            <td>0111</td>
            <td>JMP</td>
            <td>1 (M)</td>
        </tr>
        <tr>
            <td>1000</td>
            <td>JZ</td>
            <td>1 (M)</td>
        </tr>
        <tr>
            <td>1001</td>
            <td>JNZ</td>
            <td>1 (M)</td>
        </tr>
        <tr>
            <td>1010</td>
            <td>MULT</td>
            <td>2 (R, M)</td>
        </tr>
    </table>

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
- Converts `.asm` source files into `.bin` file of raw binary always and `.txt` files of ASCII `0` and `1` bits in `--debug` mode and `--pretty` mode.
- Instruction format:  
    `<4-bit opcode> [<2-bit register> <4-bit operand> [<4-bit operand3>] [<8-bit program memory address (in case of labels)>]]`

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
<img width="1619" height="502" alt="c81c3311-c1da-4d1e-92e3-f5261516a11b" src="https://github.com/user-attachments/assets/b2ff68ea-197e-4c1d-90fc-007955a14c71" />

1. **Write Assembly**

    Example: `ADD R0, R1, 0`
    - This means: add the value at memory location `0` with value at register `R1` and store the result in register `R0`.

    I have already prepared the example assembly code and is present in this repository as `index.asm`.

2. **Assemble**

    Run the assembler to convert your `.asm` file into a `.bin` file:
    ```bash
    cargo run -p assembler index.asm
    ```
    This produces raw binary in output.bin.
    
    Note:
    1. The assembler also generates a `.txt` file with ASCII `0` and `1` bits if run in debug mode.
        ```bash
        cargo run -p assembler index.asm --debug --pretty
        ```
        This produces a human-readable binary alongside the raw binary in debug.txt.
    2. We have also added a script in the root of the repository to convert the raw binary to ASCII `0` and `1` bits:
        ```bash
        python3 convertBinToASCIIBin.py output.bin
        ```
        This will print the ASCII representation of the binary to the console.

3. **Run on CPU**

    Pass output.bin to the CPU simulator:
    ```
    cargo run -p cpu output.bin
    ```
    The CPU will:
    - Load the program into instruction memory.
    - Fetch, decode, and execute each instruction.
    - Print debug output showing execution flow and final state.
### Example
**program.asm**
```
IN R1               ; Input the number
IN R0               ; Input default 1, constants are yet to be implemented
MOVEM R0 1          ; Move default 1 to memory location 1
LOOP: MOVEM R1 0    ; Support of labels; Move input to memory location 0
MULT R0 R0 0        ; Multiply value at R0 (default 1 for the first iteration) with input
SUB R1 R1 1         ; Subtract 1 from input
JNZ LOOP            ; Jump to loop if input is not 0
OUT R0              ; Output the result
```
As you might have guessed, the above program calculates the factorial of the input number.

**Output**
```
Debug mode enabled.
Loading binary file: output.bin
Binary file loaded successfully. Starting execution...
Executing instruction at PC 0: Opcode = 5, Operands = [1]
Enter value for register 1: 5
Executing instruction at PC 6: Opcode = 5, Operands = [0]
Enter value for register 0: 1
Executing instruction at PC 12: Opcode = 1, Operands = [0, 1]
Executing instruction at PC 22: Opcode = 1, Operands = [1, 0]
Executing instruction at PC 32: Opcode = 10, Operands = [0, 0, 0]
Executing instruction at PC 44: Opcode = 3, Operands = [1, 1, 1]
Executing instruction at PC 56: Opcode = 9, Operands = [22]
Executing instruction at PC 22: Opcode = 1, Operands = [1, 0]
Executing instruction at PC 32: Opcode = 10, Operands = [0, 0, 0]
Executing instruction at PC 44: Opcode = 3, Operands = [1, 1, 1]
Executing instruction at PC 56: Opcode = 9, Operands = [22]
Executing instruction at PC 22: Opcode = 1, Operands = [1, 0]
Executing instruction at PC 32: Opcode = 10, Operands = [0, 0, 0]
Executing instruction at PC 44: Opcode = 3, Operands = [1, 1, 1]
Executing instruction at PC 56: Opcode = 9, Operands = [22]
Executing instruction at PC 22: Opcode = 1, Operands = [1, 0]
Executing instruction at PC 32: Opcode = 10, Operands = [0, 0, 0]
Executing instruction at PC 44: Opcode = 3, Operands = [1, 1, 1]
Executing instruction at PC 56: Opcode = 9, Operands = [22]
Executing instruction at PC 22: Opcode = 1, Operands = [1, 0]
Executing instruction at PC 32: Opcode = 10, Operands = [0, 0, 0]
Executing instruction at PC 44: Opcode = 3, Operands = [1, 1, 1]
Executing instruction at PC 56: Opcode = 9, Operands = [22]
Executing instruction at PC 68: Opcode = 6, Operands = [0]
Output from register 0: 120
Execution completed.
Register 0: 120
Register 1: 0
Register 2: 0
Register 3: 0
Program Counter: 74
End of Execution.
```
Such output shows up if you run the CPU in debug mode i.e. with `--debug` flag:
```bash
cargo run -p cpu output.bin --debug
```

---

## Verification
The python script `convertBinToASCIIBin.py` can be used to verify the binary output by converting it to ASCII `0` and `1` bits.
Run it as follows:
```bash
python3 convertBinToASCIIBin.py output.bin
```
This will print the ASCII representation of the binary to the console, which can be compared with the expected output.

_This step is optional and mainly for debugging or cross-checking the assembler’s output._

## Current Limitations
- Input/Output is basic (manual IN and OUT instructions).
- No support for constants.

---
## Future Improvements
- Create a REPL for live assembly and execution.
- Support for more registers and larger memory space.
---
## Motivation
"Feels good to write 0s and 1s and see them do something."

This project is a practical step toward learning system software by building a CPU from scratch, understanding the fetch-decode-execute cycle, and bridging theory with a working implementation.
