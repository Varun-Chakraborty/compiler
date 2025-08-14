import java.io.*;
import java.nio.file.Files;
import java.util.Map;

class Register {
    private int regs[];
    private int count;

    public Register(int count) {
        if (count < 1 || count > 4)
            throw new IllegalArgumentException("Register count must be between 1 and 4");
        this.count = count;
        regs = new int[count];
    }

    public void set(int register, int value) {
        if (register < 0 || register > count - 1)
            throw new IllegalArgumentException("Invalid register");
        regs[register] = value;
    }

    public int get(int register) {
        if (register < 0 || register > count - 1)
            throw new IllegalArgumentException("Invalid register");
        return regs[register];
    }
}

class Memory {
    private int[] memory;

    public Memory(int size) {
        memory = new int[size];
    }

    public int size() {
        return memory.length;
    }

    public void set(int address, int value) {
        if (address < 0 || address >= memory.length) {
            throw new IndexOutOfBoundsException("Memory address out of bounds");
        }
        memory[address] = value;
    }

    public int get(int address) {
        if (address < 0 || address >= memory.length) {
            throw new IndexOutOfBoundsException("Memory address out of bounds");
        }
        return memory[address];
    }
}

class Instruction {
    private int opcode;
    private Map<Integer, Integer> symbolTable;
    private int[] operands;
    private int programCounter;

    public Instruction(Memory memory, int programCounter) {
        symbolTable = Map.of(
                0x0, 2,
                0x1, 2,
                0x2, 3,
                0x3, 3,
                0x4, 0);
        opcode = Integer.parseInt(
                String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++)),
                2);
        if (!symbolTable.containsKey(opcode)) {
            throw new IllegalArgumentException("Invalid opcode: " + opcode);
        }
        int operandCount = symbolTable.get(opcode);
        operands = new int[operandCount];
        for (int i = 0; i < operandCount; i++) {
            if (i == 0) {
                operands[i] = Integer.parseInt((String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++))), 2);
            } else {
                operands[i] = Integer.parseInt((String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++)) +
                        String.valueOf(memory.get(programCounter++))), 2);
            }
        }
        this.programCounter = programCounter;
    }

    public int getOpcode() {
        return opcode;
    }

    public int[] getOperands() {
        return operands;
    }

    public int getProgramCounter() {
        return programCounter;
    }
}

class MyCPU {
    private int programCounter;
    private int EOF = 0; // End of File
    private Memory programMemory;
    private Memory dataMemory;
    private Register register;

    public MyCPU() {
        programCounter = 0;
        register = new Register(4); // Initialize with 4 registers
        programMemory = new Memory(256); // Initialize memory with 256 addresses
        dataMemory = new Memory(256); // Initialize memory with 256 addresses
    }

    private void mover(int register, int memory) { // move to register
        this.register.set(register, this.dataMemory.get(memory));
    }

    private void movem(int register, int memory) { // move to memory
        this.dataMemory.set(memory, this.register.get(register));
    }

    private void add(int register, int sourceReg, int value) {
        this.register.set(register, this.register.get(sourceReg) + value);
    }

    private void sub(int register, int sourceReg, int value) {
        this.register.set(register, this.register.get(sourceReg) - value);
    }

    public void opcodes(int opcode, int... operands) {
        switch (opcode) {
            case 0: // MOVER
                mover(operands[0], operands[1]);
                break;
            case 1: // MOVEM
                movem(operands[0], operands[1]);
                break;
            case 2: // ADD
                add(operands[0], operands[1], operands[2]);
                break;
            case 3: // SUB
                sub(operands[0], operands[1], operands[2]);
                break;
            case 4: // HALT
                programCounter = EOF; // Set program counter to EOF
                break;
            default:
                throw new IllegalArgumentException("Invalid opcode or lesser number of operands provided");
        }
    }

    public void loadBinaryFile(String filePath) {
        try {
            String binary = Files.readString(new File(filePath).toPath());
            binary = String.join("", String.join("", binary.split("\n")).split(" "));
            int i = 0;
            if (binary.length() > programMemory.size()) {
                throw new IllegalArgumentException("Binary file exceeds memory size");
            }
            while (i < binary.length()) {
                programMemory.set(i, binary.charAt(i) - '0'); // Convert char to int
                i++;
            }
            this.EOF = i;
        } catch (Exception e) {
            System.out.println("Error loading binary file: " + e.getMessage());
        }
    }

    public void run() {
        while (programCounter < programMemory.size() && programCounter < EOF) {
            if (programCounter == EOF) {
                break; // Stop execution if EOF is reached
            }
            try {
                Instruction instruction = new Instruction(
                        programMemory,
                        programCounter);
                int opcode = instruction.getOpcode();
                int[] operands = instruction.getOperands();
                System.out.println("Executing instruction at PC " + programCounter + ": Opcode = " + opcode
                        + ", Operands = " + java.util.Arrays.toString(operands));
                programCounter = instruction.getProgramCounter();
                opcodes(opcode, operands);
            } catch (Exception e) {
                System.out.println("Error executing instruction at PC " + programCounter + ":" + e.getMessage());
                break; // Stop execution on error
            }
        }
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            // System.err.println("Please provide the path to the binary file.");
            // return;
            args = new String[1];
            args[0] = "./index.bin"; // For testing purposes, hardcoded path
        }
        MyCPU cpu = new MyCPU();
        cpu.dataMemory.set(0, 15); // Initialize memory for testing
        cpu.loadBinaryFile(args[0]);
        cpu.run();
        // read memory state
        System.out.println("Final state of data memory:");
        for (int i = 0; i < cpu.dataMemory.size(); i++) {
            if (cpu.dataMemory.get(i) != 0)
                System.out.println("Address " + i + ": " + cpu.dataMemory.get(i));
        }
        System.out.println("Final state of registers:");
        for (int i = 0; i < 4; i++) {
            System.out.println("Register " + i + ": " + cpu.register.get(i));
        }
    }
}