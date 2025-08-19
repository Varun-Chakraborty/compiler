def generateBinary(char):
    # character in integer
    char_int = ord(char)
    # convert to binary and remove '0b' prefix
    binary = bin(char_int)[2:]
    # pad with leading zeros to make it 8 bits
    padded_binary = binary.zfill(8)
    return padded_binary

def readFile(filename):
    try:
        with open(filename, 'r') as file:
            content = file.read()
            # Convert each character to its binary representation
            binary_content = ''.join(generateBinary(char) for char in content)
            print(f"Binary content of {filename}: {binary_content}")
    except FileNotFoundError:
        print(f"File {filename} not found.")
        return None

# accept command line arguments
import sys
if len(sys.argv) > 1:
    for filename in sys.argv[1:]:
        readFile(filename)
else:
    print("Please provide a file name as an argument.")
