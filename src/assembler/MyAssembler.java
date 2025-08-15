package src.assembler;

import java.io.*;
import java.nio.file.Files;
import java.util.Map;

public class MyAssembler {
    private Map<String, Integer> symbolTable;

    MyAssembler() {
        this.symbolTable = Map.of(
                "MOVER", 0,
                "MOVEM", 1,
                "ADD", 2,
                "SUB", 3,
                "HALT", 4,
                "IN", 5,
                "OUT", 6);
    }

    private String convertToBinary(String value, int size) {
        try {
            int intValue = Integer.parseInt(value);
            String binaryString = Integer.toBinaryString(intValue);
            return String.format("%0" + size + "d", Integer.parseInt(binaryString));
        } catch (NumberFormatException e) {
            throw new IllegalArgumentException("Invalid number format: " + value);
        }
    }

    private String convertToBinary(int value, int size) {
        try {
            int intValue = value;
            String binaryString = Integer.toBinaryString(intValue);
            return String.format("%0" + size + "d", Integer.parseInt(binaryString));
        } catch (NumberFormatException e) {
            throw new IllegalArgumentException("Invalid number format: " + value);
        }
    }

    private String assemble(String filePath) {
        // create or open the binary file
        String path = "output.bin";
        File binaryFile = new File(path);
        if (binaryFile.exists()) {
            binaryFile.delete(); // clear the file if it exists
        }
        BufferedWriter writer = null;
        try {
            writer = new BufferedWriter(new FileWriter(binaryFile, true));
            System.out.println("Starting assembly for file: " + filePath);
            String assembly = Files.readString(new File(filePath).toPath());
            String statements[] = assembly.split("\n");
            for (String statement : statements) {
                statement = statement.split(";")[0];
                statement = statement.trim();

                if (statement.isEmpty() || statement.startsWith(";")) {
                    continue; // Skip empty lines and comments
                }
                statement = statement.split(";")[0];
                statement = statement.trim();
                System.out.println("Processing statement: " + statement);
                String opcode = statement.split(" ")[0];
                if (!symbolTable.containsKey(opcode)) {
                    throw new IllegalArgumentException("Unknown opcode: " + opcode + " in statement: " + statement);
                }
                writer.write(convertToBinary(symbolTable.get(opcode), 4));
                try {
                    String operands[] = statement.substring(statement.indexOf(' ') + 1).split(",");
                    for (int i = 0; i < operands.length; i++) {
                        operands[i] = operands[i].trim();
                        if (operands[i].startsWith("R")) {
                            String regNum = convertToBinary(operands[i].substring(1), 2);
                            writer.write(regNum);
                        } else if (operands[i].matches("\\d+")) {
                            writer.write(convertToBinary(operands[i], 4));
                        } else {
                            throw new IllegalArgumentException("Invalid operand: " + operands[i]);
                        }
                    }
                } catch (Exception e) {
                    System.out.println("No operands found for statement: " + statement);
                }
            }
        } catch (IOException e) {
            System.err.println("Error reading assembly file: " + e.getMessage());
            return null;
        } finally {
            try {
                if (writer != null) {
                    writer.close();
                }
            } catch (IOException e) {
                System.err.println("Error closing writer: " + e.getMessage());
            }
        }
        return path;
    }

    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Please provide the path to the assembly file.");
            return;
        }
        MyAssembler assembler = new MyAssembler();
        String binaryFilePath = assembler.assemble(args[0]);
        System.out.println("Binary file created at: " + binaryFilePath);
    }
}