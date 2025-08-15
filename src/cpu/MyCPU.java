package src.cpu;

import java.io.*;
import java.nio.file.Files;
import java.util.Map;

class Register {
    private byte regs[];
    private int count;

    public Register(int count) {
        if (count < 1 || count > 4)
            throw new IllegalArgumentException("Register count must be between 1 and 4");
        this.count = count;
        regs = new byte[count];
    }

    public void set(int register, int value) {
        if (register < 0 || register > count - 1)
            throw new IllegalArgumentException("Invalid register");
        regs[register] = (byte) value;
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
                0x4, 0,
                0x5, 1,
                0x6, 1);
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
            if (i == 0 || (i == 1 && (opcode == 2 || opcode == 3))) {
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

public class MyCPU {
    private int programCounter;
    private int EOF = 0; // End of File
    private Memory programMemory;
    private Memory dataMemory;
    private Register register;
    private boolean debug;

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

    private void add(int register, int sourceReg, int memAdd) {
        this.register.set(register, this.register.get(sourceReg) + dataMemory.get(memAdd));
    }

    private void sub(int register, int sourceReg, int memAdd) {
        this.register.set(register, this.register.get(sourceReg) - dataMemory.get(memAdd));
    }

    private void in(int register) {
        try {
            BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
            System.out.print("Enter value for register " + register + ": ");
            String input = reader.readLine();
            int value = Integer.parseInt(input);
            this.register.set(register, value);
        } catch (IOException | NumberFormatException e) {
            System.out.println("Error reading input: " + e.getMessage());
        }
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
            case 5: // IN
                in(operands[0]);
                break;
            case 6: // OUT
                System.out.println("Output from register " + operands[0] + ": " + register.get(operands[0]));
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

    public void run(boolean debug) {
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
                if (debug) {
                    System.out.println("Executing instruction at PC " + programCounter + ": Opcode = " + opcode
                            + ", Operands = " + java.util.Arrays.toString(operands));
                }
                programCounter = instruction.getProgramCounter();
                opcodes(opcode, operands);
            } catch (Exception e) {
                System.out.println("Error executing instruction at PC " + programCounter + ": " + e.getMessage());
                break; // Stop execution on error
            }
        }
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Please provide the path to the binary file.");
            return;
        }
        MyCPU cpu = new MyCPU();
        cpu.loadBinaryFile(args[0]);
        System.out.println("Binary file loaded successfully. Starting execution...");
        cpu.debug = args.length > 1 && args[1].equals("--debug");
        if (cpu.debug)
            System.out.println("Debug mode enabled.");
        cpu.run(cpu.debug);
        if (cpu.debug) {
            System.out.println("Execution completed.");
            System.out.println("Final Register State: ");
            for (int i = 0; i < 4; i++) {
                System.out.println("Register " + i + ": " + cpu.register.get(i));
            }
            System.out.println("Program Counter: " + cpu.programCounter);
            System.out.println("End of Execution.");
        }
    }
}