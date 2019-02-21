            mvn r6, #0       ; r6 = -1
            mov r7, #0       ; initialize loop r7 = 0
            b   TestForDone  ; test for done at end of loop!
DoFor       ldr r8, [r4, r7] ; get element array[r7]
            cmp r8, r3       ; element == value of interest
            bne IncR7        ; No  - continue loop
            mov r6, r7       ; Yes - save index
            b   DoneFor      ;     - break
IncR7       add r7, r7, #1   ; r7++
TestForDone cmp r7, r5       ; r7 < r5
            blt DoFor        ; Yes - dop loop body again
            DoneFor          ; continue 
