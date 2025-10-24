IN R1               ; Input the number
MOVERI R0, 1        ; Move 1 to R0
LOOP: MOVEM R1, 0   ; Support of labels; Move input to memory location 0
MULT R0, 0          ; Multiply value at R0 (default 1 for the first iteration) with input
MOVER R1, 0         ; Move value at memory location 0 to R1
SUBI R1, 1          ; Subtract 1 from input
JNZ LOOP            ; Jump to loop if input is not 0
OUT R0              ; Output the result
HALT                ; END of program
