
    // find the offset to start our tuple list at
    add UINT_START, x1, x0
    // even -> list 1, odd -> list 2

parse_line_start:
    //cmp x0, UINT_START
    //b.eq assemble_radix
parse_line_left:
    ldrb w4, [x0], #1   // load x0 into register x2, then r0 += 1

    sub x4, x4, #48     // b'0' -> 48
    mov x0, #5





assemble_radix
    // PAIR_COUNT
    // parse the numbers
    // start number, accumulate until space
    // pass space
    // start number, accumulate until newline
    // continue until end of memory address

    // 
    add x1, x1, x0      // x1 now contains ending memory address
    mov x3, #0          // accumulate total
    mov x4, #0          // right
    mov x5, #0          // index
find_left:
    cmp x0, x1          // check if we're done
    b.eq finish

    ldrb w2, [x0], #1   // load x0 into register x2, then r0 += 1
    sub x2, x2, #48     // check if ASCII
    cmp x2, #9
    bgt find_left       // if not digit, repeat loop

    mov  x4, x2         // store the left digit as a potential right digit
    mov x5, #10
    mul x2, x2, x5
    add x3, x3, x2
find_end:
    ldrb w2, [x0], #1
                        // check if '\n'
    cmp x2, #10         // check if '\n\'
    beq accumulate

    sub x2, x2, #48     // check if a digit
    cmp x2, #9
    bgt find_end

    mov x4, x2          // store as rightmost digit
    b find_end

accumulate:             // add the rightmost digit to the accumulator
    add x3, x3, x4
    b find_left

finish:                 // load the accumulator into the return register
    mov x0, x3
    ret
