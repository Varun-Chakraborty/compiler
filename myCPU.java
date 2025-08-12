import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

class Register {
    private int regs[];
    private int count;
    public Register(int count) {
        if (count < 1 || count > 4) throw new IllegalArgumentException("Register count must be between 1 and 4");
        this.count = count;
        this.regs = new int[count];
    }
    public void set(int register, int value) {
        if (register < 0 || register > this.count-1) throw new IllegalArgumentException("Invalid register");
        this.regs[register] = value;
    }
    public int get(int register) {
        if (register < 0 || register > this.count-1) throw new IllegalArgumentException("Invalid register");
        return this.regs[register];
    }
}

class Memory {
    private int[] memory;

    public Memory(int size) {
        this.memory = new int[size];
    }

    public void set(int address, int value) {
        if (address < 0 || address >= memory.length) {
            throw new IndexOutOfBoundsException("Memory address out of bounds");
        }
        this.memory[address] = value;
    }

    public int get(int address) {
        if (address < 0 || address >= memory.length) {
            throw new IndexOutOfBoundsException("Memory address out of bounds");
        }
        return this.memory[address];
    }
}

class MyCPU {
    private int programCounter;
    private Memory memory;
    private Register register;

    public MyCPU() {
        this.programCounter = 0;
        this.register = new Register(4); // Initialize with 4 registers
        this.memory = new Memory(256); // Initialize memory with 256 addresses
    }

    private void mover(int register, int memory) { // move to register
        this.register.set(register, this.memory.get(memory));
        this.programCounter++;
    }

    private void movem(int register, int memory) { // move to memory
        this.memory.set(memory, this.register.get(register));
        this.programCounter++;
    }

    private void add(int register, int sourceReg, int value) {
        this.register.set(register, this.register.get(sourceReg) + value);
        this.programCounter++;
    }

    private void sub(int register, int sourceReg, int value) {
        this.register.set(register, this.register.get(sourceReg) - value);
        this.programCounter++;
    }

    public void opcodes(int opcode, int operand1, int operand2) {
        switch (opcode) {
            case 0: // MOVER
                this.mover(operand1, operand2);
                break;
            case 1: // MOVEM
                this.movem(operand1, operand2);
                break;
            default:
                throw new IllegalArgumentException("Invalid opcode or lesser number of operands provided");
        }
    }

    public void opcodes(int opcode, int operand1, int operand2, int operand3) {
        switch (opcode) {
            case 2: // ADD
                this.add(operand1, operand2, operand3);
                break;
            case 3: // SUB
                this.sub(operand1, operand2, operand3);
                break;
            default:
                throw new IllegalArgumentException("Invalid opcode");
        }
    }

    public void loadBinaryFile(String filePath) {
        try {
            String content = new String(Files.readAllBytes(Paths.get(filePath)));
            String[] instructions = content.split("\n");
            if (instructions.length == 0) {
                throw new IllegalArgumentException("No instructions found in the file");
            }
            // Process each instruction
            this.programCounter = 0; // Reset program counter before execution
            for (int i = 0; i < instructions.length; i++) {
                String instruction[] = instructions[i].split(" ");
                if (instruction.length < 3 || instruction.length > 4) {
                    throw new IllegalArgumentException("Invalid instruction length: " + instruction);
                }
                int opcode = Integer.parseInt(instruction[0], 2);
                int operand1 = Integer.parseInt(instruction[1], 2);
                int operand2 = Integer.parseInt(instruction[2], 2);
                if (instruction.length > 3) {
                    int operand3 = Integer.parseInt(instruction[3], 2);
                    this.opcodes(opcode, operand1, operand2, operand3);
                } else {
                    this.opcodes(opcode, operand1, operand2);
                }
            }
        } catch (IOException e) {
            e.printStackTrace();
        } catch (NumberFormatException e) {
            System.err.println("Error parsing binary file: " + e.getMessage());
        } catch (IllegalArgumentException e) {
            System.err.println("Error in opcode or operands: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Please provide the path to the binary file.");
            return;
        }
        MyCPU cpu = new MyCPU();
        cpu.memory.set(0, 15); // Initialize memory for testing
        cpu.loadBinaryFile(args[0]);
        System.out.println("Program executed successfully.");
        System.out.println("Final state of registers:");
        for (int i = 0; i < 4; i++) {
            System.out.println("Register " + (char)(i+65) + ": " + cpu.register.get(i));
        }
        System.out.println("Program Counter: " + cpu.programCounter);
        System.out.println("Memory contents:");
        // Print non-zero memory contents
        boolean isThereAnyNonZero = false;
        for (int i = 0; i < 256; i++) {
            if (cpu.memory.get(i) != 0) {
                isThereAnyNonZero = true;
                System.out.println("Memory[" + i + "] = " + cpu.memory.get(i));
            }
        }
        if (!isThereAnyNonZero) {
            System.out.println("Memory is empty.");
        }
    }
}