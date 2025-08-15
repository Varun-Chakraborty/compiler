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

    Example: `ADD R0, R1, 0`
    - This means: add the value at memory location `0` with value at register `R1` and store the result in register `R0`.

    I have already prepared the example assembly code and is present in this repository as `index.asm`.
2. **Compile the machines**
    ```bash
    javac src/assembler/*.java src/cpu/*.java
    ```
3. **Assemble**

    Run the assembler to convert your `.asm` file into a `.bin` file:
    ```bash
    java src.assembler.MyAssembler index.asm
    ```
    This produces output.bin containing only '0' and '1' characters.
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
IN R2           ; input to register 2
MOVEM R2, 0     ; move to memory address 0 from register 2
IN R2           ; input to register 2
MOVEM R2, 1     ; move to memory address 1 from register 2
MOVER R1, 0     ; move to register 1 from memory address 0
ADD R0, R1, 1   ; add value stored in register 1 to value stored in memory address 1 and store in register 0
MOVEM R0, 0     ; move to memory address 0 from register 0
MOVER R3, 0     ; move to register 3 from memory address 0
OUT R3          ; output the value stored in register 3
HALT            ; end the program
```
**Output**
```
Binary file loaded successfully. Starting execution...
Debug mode enabled.
Executing instruction at PC 0: Opcode = 5, Operands = [2]
Enter value for register 2: 15
Executing instruction at PC 6: Opcode = 1, Operands = [2, 0]
Executing instruction at PC 16: Opcode = 5, Operands = [2]
Enter value for register 2: 15
Executing instruction at PC 22: Opcode = 1, Operands = [2, 1]
Executing instruction at PC 32: Opcode = 0, Operands = [1, 0]
Executing instruction at PC 42: Opcode = 2, Operands = [0, 1, 1]
Executing instruction at PC 54: Opcode = 1, Operands = [0, 0]
Executing instruction at PC 64: Opcode = 0, Operands = [3, 0]
Executing instruction at PC 74: Opcode = 6, Operands = [3]
Output from register 3: 30
Executing instruction at PC 80: Opcode = 4, Operands = []
Execution completed.
Final Register State: 
Register 0: 30
Register 1: 15
Register 2: 15
Register 3: 30
Program Counter: 84
End of Execution.
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
