IN R2               ; Input the number
OUT R2
MOVEI R0, 1         ; Move 1 to R0
LOOP: MULT R0, R2   ; Support of labels; Multiply value at R0 (default 1 for the first iteration) with input
SUBI R2, 1          ; Subtract 1 from input
JNZ LOOP            ; Jump to loop if input is not 0
OUT R0              ; Output the result
HALT                ; END of program
