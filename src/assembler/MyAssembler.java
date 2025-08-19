package src.assembler;

import java.io.BufferedInputStream;
import java.io.BufferedOutputStream;
import java.io.BufferedWriter;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Map;

class Writer {
    private boolean debug;
    private boolean pretty;
    private BufferedOutputStream outputStream;
    private BufferedWriter writer;
    private byte buffer;
    private int bufferSize = 0;
    private static final int BUFFER_SIZE = 8; // Size of the buffer in bits

    public Writer(boolean debug, boolean pretty) throws IOException {
        this.debug = debug;
        this.pretty = pretty;
        this.outputStream = new BufferedOutputStream(new FileOutputStream("output.bin"));
        if (debug) {
            writer = new BufferedWriter(new FileWriter("output.txt"));
        }
    }

    private void flush() throws IOException {
        if (bufferSize > 0) {
            if (bufferSize < BUFFER_SIZE) {
                // Pad the buffer with zeros if it is not full
                buffer <<= (BUFFER_SIZE - bufferSize);
            }
            // Write the buffer to the output stream
            outputStream.write(buffer);
            buffer = 0; // Reset buffer
            bufferSize = 0; // Reset buffer size
        }
        outputStream.flush();
        if (debug) {
            writer.flush();
        }
    }

    public void addToBuffer(int data, int bitCount) {
        while (bitCount + bufferSize > BUFFER_SIZE) {
            // split the data into two parts, will the buffer will first part, flush the
            // buffer, and then add the second part
            int remainingBits = BUFFER_SIZE - bufferSize;
            buffer = (byte) (buffer << remainingBits | data >> (bitCount - remainingBits));
            bufferSize += remainingBits;
            data &= (1 << (bitCount - remainingBits)) - 1; // Keep the remaining bits
            bitCount -= remainingBits;
            try {
                flush();
            } catch (IOException e) {
                System.err.println("Error flushing buffer: " + e.getMessage());
            }
        }
        buffer = (byte) (buffer << bitCount | data);
        bufferSize += bitCount;
    }

    public void write(int data, int bitCount) {
        try {
            addToBuffer(data, bitCount);
            if (debug) {
                // Write the binary representation to the debug file padded upto bitCount
                String binaryString = Integer.toBinaryString(data);
                String paddedBinaryString = String.format("%" + bitCount + "s", binaryString).replace(' ', '0');
                writer.write(paddedBinaryString);
                if (pretty) writer.write(' ');
            }
        } catch (IOException e) {
            System.err.println("Error writing data: " + e.getMessage());
        }
    }

    public void newLine() {
        try {
            if (debug && pretty) {
                writer.newLine();
            }
        } catch (IOException e) {
            System.err.println("Error writing new line: " + e.getMessage());
        }
    }

    public void close() throws IOException {
        flush();
        outputStream.close();
        if (debug) {
            writer.close();
        }
    }
}

class Instruction {
    private int opcode;
    private String[] operands;
    private int operandCount;
    private int currentOperand;
    private Map<String, Integer> symbolTable;
    private Writer writer;

    public Instruction(Map<String, Integer> symbolTable, boolean debug, boolean pretty) {
        this.symbolTable = symbolTable;
        this.opcode = 0;
        this.operands = new String[3]; // Assuming max 3 operands
        this.operandCount = 0;
        this.currentOperand = 0;
        try {
            this.writer = new Writer(debug, pretty);
        } catch (IOException e) {
            throw new RuntimeException("Error initializing writer: " + e.getMessage());
        }
    }

    public void setOpcode(String opcode) {
        if (!symbolTable.containsKey(opcode)) {
            throw new IllegalArgumentException("Unknown opcode: " + opcode);
        }
        this.opcode = symbolTable.get(opcode);
        this.operandCount = (this.opcode == 2 || this.opcode == 3) ? 3
                : (this.opcode == 0 || this.opcode == 1) ? 2 : (this.opcode == 5 || this.opcode == 6) ? 1 : 0;
    }

    public void addOperand(String operand) {
        if (operand.isEmpty()) {
            return;
        }
        if (operandCount <= currentOperand) {
            throw new IllegalArgumentException("Too many operands for instruction: " + opcode);
        }
        operands[currentOperand++] = operand;
    }

    public boolean isEmpty() {
        return opcode == 0 && operandCount == 0;
    }

    public boolean isInComplete() {
        return operandCount > 0 && currentOperand < operandCount;
    }

    public void done() {
        if (isEmpty()) {
            return;
        }
        if (isInComplete()) {
            throw new IllegalStateException("Instruction is incomplete");
        }
        writer.write(opcode, 4);
        for (int i = 0; i < operandCount; i++) {
            // check if the operand is a number
            if (operands[i].matches("\\d+")) {
                writer.write(Integer.parseInt(operands[i]), 4);
            } else if (operands[i].charAt(0) == 'R' && operands[i].substring(1).matches("\\d+")) {
                writer.write(Integer.parseInt(operands[i].substring(1)), 2);
            } else {
                throw new IllegalArgumentException("Invalid operand: " + operands[i]);
            }
        }
        // reset the instruction
        opcode = 0;
        operandCount = 0;
        currentOperand = 0;
        operands = new String[3];
        writer.newLine();
    }

    public void close() {
        try {
            writer.close();
        } catch (IOException e) {
            throw new RuntimeException("Error closing writer: " + e.getMessage());
        }
    }
}

public class MyAssembler {
    private Map<String, Integer> symbolTable;
    private Instruction instruction;

    MyAssembler(boolean debug, boolean pretty) {
        this.symbolTable = Map.of(
                "MOVER", 0,
                "MOVEM", 1,
                "ADD", 2,
                "SUB", 3,
                "HALT", 4,
                "IN", 5,
                "OUT", 6);
        this.instruction = new Instruction(symbolTable, debug, pretty);
    }

    private void assemble(String filePath) {
        BufferedInputStream reader = null;
        try {
            System.out.println("Starting assembly for file: " + filePath);
            reader = new BufferedInputStream(new FileInputStream(filePath));
            StringBuilder token = new StringBuilder();
            while (reader.available() > 0) {
                int c = reader.read();
                if (c == -1) {
                    break;
                }
                char ch = (char) c;
                if (ch == ',' || ch == ' ' || ch == '\t') {
                        if (instruction.isEmpty()) {
                            instruction.setOpcode(token.toString());
                        } else {
                            instruction.addOperand(token.toString());
                        }
                } else if (ch == ';' || ch == '\n') {
                    if (instruction.isEmpty()) {
                        instruction.setOpcode(token.toString());
                    } else {
                        instruction.addOperand(token.toString());
                    }
                    instruction.done();
                    if (ch == ';') {
                        while (ch != '\n' && reader.available() > 0) {
                            ch = (char) reader.read();
                        }
                    }
                } else {
                    token.append(ch);
                    continue;
                }
                token.setLength(0);
            }
        } catch (IOException e) {
            System.err.println("Error reading assembly file: " + e.getMessage());
        } finally {
            try {
                reader.close();
                instruction.close();
            } catch (IOException e) {
                System.err.println("Error closing writer: " + e.getMessage());
            }
        }
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Please provide the path to the assembly file.");
            return;
        }
        boolean debug = false, pretty = false;
        if (args.length > 1 && args[1].equals("--debug")) {
            debug = true;
        }
        if (args.length > 2 && args[2].equals("--pretty")) {
            pretty = true;
        }
        if (debug) {
            System.out.println("Debug mode enabled.");
        }
        if (pretty) {
            System.out.println("ASCII binary would be prettified.");
        }
        MyAssembler assembler = new MyAssembler(debug, pretty);
        assembler.assemble(args[0]);
        System.out.println("Binary file created at: output.bin");
        if (debug) {
            System.out.println("Debug file created at: output.txt");
        }
    }
}