package src.cpu;

import java.io.BufferedInputStream;
import java.io.BufferedReader;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStreamReader;
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

    public void set(int register, byte value) {
        if (register < 0 || register > count - 1)
            throw new IllegalArgumentException("Invalid register");
        regs[register] = (byte) value;
    }

    public byte get(int register) {
        if (register < 0 || register > count - 1)
            throw new IllegalArgumentException("Invalid register");
        return regs[register];
    }
}

class Memory {
    private byte[] memory;

    public Memory(int size) {
        memory = new byte[size];
    }

    public int size() {
        return memory.length;
    }

    public void set(int address, byte value) {
        if (address < 0 || address >= memory.length) {
            throw new IndexOutOfBoundsException("Memory address out of bounds");
        }
        memory[address] = value;
    }

    public byte get(int address) {
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
    private int EOF; // End of File
    private Memory programMemory;
    private Memory dataMemory;
    private Register register;
    private boolean debug;

    public MyCPU() {
        programCounter = 0;
        EOF = 0;
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
        this.register.set(register, (byte) (this.register.get(sourceReg) + dataMemory.get(memAdd)));
    }

    private void sub(int register, int sourceReg, int memAdd) {
        this.register.set(register, (byte) (this.register.get(sourceReg) - dataMemory.get(memAdd)));
    }

    private void in(int register) {
        try {
            BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
            System.out.print("Enter value for register " + register + ": ");
            String input = reader.readLine();
            byte value = Byte.parseByte(input);
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
        BufferedInputStream inputStream = null;
        try {
            inputStream = new BufferedInputStream(new FileInputStream(filePath));
            while (inputStream.available() > 1) {
                byte byteRead = (byte) inputStream.read();
                for(int i = 0; i < 8; i++) {
                    byte bit = (byte) ((byteRead >> (7 - i)) & 1);
                    programMemory.set(programCounter++, bit);
                    if (programCounter >= programMemory.size()) {
                        System.out.println("Program memory overflow. Stopping load.");
                        return;
                    }
                }
            }
            EOF = inputStream.read(); // Read EOF byte; last byte indicates EOF
            programCounter = 0; // Reset program counter after loading
            System.out.println("Binary file loaded successfully. Starting execution...");

        } catch (IOException e) {
            System.out.println("Error loading binary file: " + e.getMessage());
        } finally {
            try {
                inputStream.close();
            } catch (IOException e) {
                System.out.println("Error closing input stream: " + e.getMessage());
            }
        }
    }

    public void run() {
        while (programCounter < programMemory.size() && programCounter < EOF) {
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
        cpu.debug = args.length > 1 && args[1].equals("--debug");
        if (cpu.debug)
            System.out.println("Debug mode enabled.");
        cpu.loadBinaryFile(args[0]);
        cpu.run();
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