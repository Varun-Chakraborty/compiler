IN R2; input to register 2
MOVEM R2, 0; move to memory address 0 from register 2
MOVER R1, 0; move to register 1 from memory address 0
ADD R0, 1, 5; add value stored in register 1 and value 5 and store the result in register 2
MOVEM R0, 0; move to memory address 0 from register 0
MOVER R3, 0; move to register 3 from memory address 0
OUT R3; output the value stored in register 3
HALT; end the program