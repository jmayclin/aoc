    .global day8_p1
    .type day8_p1, "function"
    .p2align 4

// x0 <- pointer to start
// x1 <- pointer to end of text
day8_p1:
    sub sp, sp, #16
    add x1, x0, x1          // x0 is pointer to start of text, x1 is length
                            // so now x1 is a pointer to the end of the text.
    mov x2, x0              // char ptr, count how many entries
    mov x4, #0              // accumulator
advance_to_map:
    ldrb w3, [x2], #1
    cmp x3, #10             // b'\n' == 10
    b.ne advance_to_map
    add x2, x2, #1             // skip the final newline

// we build a map that directly maps three letter codes to
// a location in memory, and use that to hop around
// example -> PGQ = (QRB, MJB)
// index = P * 26^2 + G * 26 + Q
// left = Q * 26^2 + R * 26 + B
// right = ...
// sp[index] = sp + left
// sp[index + 8] = sp + right

// how much of an impact would there be to splitting 
// my key assembly out into a function. I'd assume not much?
    mov x7, #26
build_map:
    cmp x1, x2
    b.eq find_zzz  // map is fully built, start iterating
    // x4 -> key
    // x5 -> left
    // x6 -> right
    // key - 1
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    mul x3, x3, x7          // key = 'C' * 26
    mul x3, x3, x7          
    mov x4, x3              // first character
    // key - 2
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    mul x3, x3, x7          // key = 'C' * 26
    add x4, x4, x3
    // key - 3
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    add x4, x4, x3

    add x2, x2, #4          // skip the " = ("

    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    mul x3, x3, x7          // key = 'C' * 26
    mul x3, x3, x7          
    mov x5, x3              // first character
    // key - 2
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    mul x3, x3, x7          // key = 'C' * 26
    add x5, x5, x3
    // key - 3
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    add x5, x5, x3

    add x2, x2, #2

    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    mul x3, x3, x7          // key = 'C' * 26
    mul x3, x3, x7          
    mov x6, x3              // first character
    // key - 2
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    mul x3, x3, x7          // key = 'C' * 26
    add x6, x6, x3
    // key - 3
    ldrb w3, [x2], #1
    sub x3, x3, #65         // b'A' == 65
    add x6, x6, x3

    add x2, x2, #2

    // store the values into the map
    lsl x4, x4, #4      // multiply by 16 (2 * 8 byte pointers)
    lsl x5, x5, #4      // multiply by 16 (2 * 8 byte pointers)
    lsl x6, x6, #4      // multiply by 16 (2 * 8 byte pointers)


    //    *(sp - (index * 16)) = sp - (left * 16)

    mov x8, sp
    sub x4, x8, x4     // memory address of key slot
    sub x5, x8, x5
    sub x6, x8, x6
    stp x5, x6, [x4]
    b build_map
    // read in the first index
    // convert to memory value
    // 
find_zzz:
    mov x2, x0 // reset "instruction pointer" to start of text
    mov x3, sp // set pointer to AAA
    mov x4, #0 // set step counter to 0
    mov x5, #25 // figure out address of ZZZ
    mul x5, x5, x7
    mul x5, x5, x7
    mov x6, #25
    mul x6, x6, x7
    add x5, x5, x6
    add x5, x5, #25
    lsl x5, x5, #4
    sub x5, x3, x5

graph_traversal:
    ldrb w6, [x2], #1   // load in 'L' or 'R'
    cmp x6, #10  // if instruction pointer is at newline b'\n' = 10
    b.ne graph_continue
    mov x2, x0  // reset "instruction pointer"
    ldrb w6, [x2], #1   // load in 'L' or 'R'
graph_continue:
    cmp x3, x5  // map pointer is ZZZ
    b.eq finish
    cmp x6, #76 // b'L' = 76
    b.eq graph_deref
    add x3, x3, #8  // we need the right side
graph_deref:
    ldr x3, [x3]
    add x4, x4, #1
    b graph_traversal
    // ldr x0, [sp, #8]      // using this to check my math
    // sub x0, x8, x0 // sp - (sp - (left * 16) )
    // lsr x0, x0, #4

finish:
    mov x0, x4
    add sp, sp, #16
    ret
    // check if insp is at end, then loop back around
    // check if at ZZZ, then b finish

    // mov x0, #0       // Set x0 to 0 for brk
    // ldr x1, =4096     // Set x1 to the size of memory to allocate (e.g., 4096 bytes)
    // mov x8, #214      // System call number for brk on AArch64 (syscall: 0xD6)


