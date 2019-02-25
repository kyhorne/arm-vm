; pgrm.asm
; Search an array to see if it contains a known value.

; Assumptions:
; R4 initially contains the start address of the array.
; R5 initially contains the number of elements in the array.
; R3 initially contains the “value of interest”.
; If the “value of interest” is in the array, then at the end of execution, R6
; will contain the index of the element that contains the “value of interest”.
; If the “value of interest” is not in the array, then at the end of execution,
; R6 will contain –1.

; Equivalent C-like pseudo-code:
; r6 = -1 // Initially, the value has not yet been.
; for(r7 = 0, r7 < r5, r7++) // r5 == array size
; {
;   if(array[r7] == r3) // Found!
;   {
;      r6 = r7; // Save index.
;      break;   // Exit loop.
;   }
; }

            mov r5, #3       ; # Elements in array.
            mov r4, #0x1234  ; Start address
            mov r3, #'a'     ; Value of interest.
            str r3, [r4, #2] ; Store value of interest at array[2].

            mvn r6, #0       ; r6 = -1
            mov r7, #0       ; Initialize loop r7 = 0.
            bal TestForDone  ; Test for done at end of loop!
DoFor       ldr r8, [r4, r7] ; Get element array[r7]
            cmp r8, r3       ; Element == value of interest.
            bne IncR7        ; No  - Continue loop.
            mov r6, r7       ; Yes - Save index.
            bal DoneFor      ;     - Break.
IncR7       add r7, r7, #1   ; r7++
TestForDone cmp r7, r5       ; r7 < r5
            blt DoFor        ; Yes - Do loop body again.
            DoneFor          ; Continue.
