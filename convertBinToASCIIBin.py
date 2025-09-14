# convert integer byte value to binary
def generateBinary(byte_value):
    # convert to binary and remove '0b' prefix
    binary = bin(byte_value)[2:]
    # pad with leading zeros to make it 8 bits
    padded_binary = binary.zfill(8)
    return padded_binary

def readFile(filename):
    try:
        with open(filename, 'rb') as file:
            content = file.read()
            # Convert each byte to its binary representation
            binary_content = ''.join(generateBinary(byte_value) for byte_value in content)
            print(f"Binary content of {filename}:\n{binary_content}")
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
