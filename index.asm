IN R0               ; Input the number
MOVEM R0, 1         ; Move input to memory location 1
MOVER R1, 1         ; Move input value at memory location 1 to R1
DC 1, 1              ; Constants; declare a constant of value 1 at memory location 1
MOVER R0, 1         ; Move value at memory location 1 i.e. 1 to R0
LOOP: MOVEM, R1, 0  ; Support of labels; Move input to memory location 0
MULT R0, 0          ; Multiply value at R0 (default 1 for the first iteration) with input
SUB R1, 1           ; Subtract 1 (at memory location 1) from input
JNZ LOOP            ; Jump to loop if input is not 0
OUT R0              ; Output the result
HALT                ; END of program
