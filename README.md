# Compiler

## Overview
This project is a **from-scratch CPU simulator** paired with a simple **assembler** that can translate custom assembly language into machine code (represented as ASCII 0/1 bits).

The CPU executes basic instructions like data movement, arithmetic, and halting.  
The assembler converts human-readable assembly into a `.bin` file, which the CPU can then run.

This project is being built to learn **system software** and understand **how CPUs work** at a low level.

---

## Features

### CPU
- Executes a custom instruction set.
- Supports:
    - `MOVER` — Move from memory to register.
    - `MOVEM` — Move from register to memory.
    - `ADD` — Add a memory value to a register.
    - `SUB` — Subtract a memory value from a register.
    - `HALT` — Stop execution.
    - `IN` — Load a value into a register.
    - `OUT` — Output a value from a register.
- Keeps track of:
    - **Registers** (R0, R1, R2, R3)
    - **Data memory**
    - **Program counter (PC)**

### Assembler
- Converts `.asm` source files into `.bin` files of ASCII `0` and `1` bits.
- Instruction format:  
    `<4-bit opcode> [<2-bit register> <4-bit operand> [<4-bit operand3>]]`
- Symbol table mapping for opcodes (categorised based on argument it takes):

    1. **No Argument**
    ```
    HALT -> 4
    ```
    2. **One Argument**
    ```
    IN -> 5
    OUT -> 6
    ```
    3. **Two arguments**
    ```
    MOVER -> 0
    MOVEM -> 1
    ```
    4. **Three arguments**
    ```
    ADD -> 2
    SUB -> 3
    ```
---

## How It Works
<img width="1619" height="502" alt="c81c3311-c1da-4d1e-92e3-f5261516a11b" src="https://github.com/user-attachments/assets/b2ff68ea-197e-4c1d-90fc-007955a14c71" />

1. **Write Assembly**

    Example: `ADD R0, 5`
    - This means: add the value at memory location `5` to register `R0`.
2. **Compile the machines**
    ```bash
    javac src/assembler/*.java src/cpu/*.java
    ```
3. **Assemble**

    Run the assembler to convert your `.asm` file into a `.bin` file:
    ```bash
    java src.assembler.MyAssembler index.asm
    This produces output.bin containing only '0' and '1' characters.
    ```
4. **Run on CPU**

    Pass output.bin to the CPU simulator:
    ```
    java src.cpu.MyCPU output.bin
    ```
    The CPU will:
    - Load the program into instruction memory.
    - Fetch, decode, and execute each instruction.
    - Print debug output showing execution flow and final state.
### Example
**program.asm**
```
IN R0, 5       ; Load value 5 into R0
ADD R0, 1      ; Add memory[1] to R0
MOVEM R0, 0    ; Store R0 into memory[0]
HALT           ; Stop execution
```
**Output**
```
Executing instruction at PC 0: Opcode = 5, Operands = [0, 5]
Executing instruction at PC 10: Opcode = 2, Operands = [0, 1, 5]
Executing instruction at PC 24: Opcode = 1, Operands = [0, 0]
Executing instruction at PC 34: Opcode = 4, Operands = []
Final state of data memory:
Address 0: 5
Final state of registers:
Register 0: 5
Register 1: 0
Register 2: 0
Register 3: 0
```
---
## Current Limitations
- `.bin` file is not true binary — it stores ASCII '0'/'1' characters (1 byte each).
- No labels or jumps — programs execute sequentially.
- No branching or conditional execution.
- Input/Output is basic (manual IN and OUT instructions).
---
## Future Improvements
- Pack bits into actual binary format to reduce file size.
- Add labels and a symbol resolution system.
- Implement branching/jump instructions.
- Create a REPL for live assembly and execution.
- Support for more registers and larger memory space.
---
## Motivation
"Feels good to write 0s and 1s and see them do something."

This project is a practical step toward learning system software by building a CPU from scratch, understanding the fetch-decode-execute cycle, and bridging theory with a working implementation.
