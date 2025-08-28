IN R1               ; Input the number
IN R0               ; Input default 1, constants are yet to be implemented
MOVEM R0 1          ; Move default 1 to memory location 1
LOOP: MOVEM R1 0    ; Support of labels; Move input to memory location 0
MULT R0 R0 0        ; Multiply value at R0 (default 1 for the first iteration) with input
SUB R1 R1 1         ; Subtract 1 from input
JNZ LOOP            ; Jump to loop if input is not 0
OUT R0              ; Output the result
