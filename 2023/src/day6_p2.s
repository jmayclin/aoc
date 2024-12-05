    .global day6_p2
    .type day6_p2, "function"
    .p2align 4

// hmm, I think it's really about time I learned
// how to call a function. I don't think it's fair to
// call this "idomatic" assembly. It is very ugly
day6_p2:
    mov x10, #10
    mov x1, #0          // time
    mov x2, #0          // distance
    add x0, x0, #5      // "Time:"
parse_time:
    ldrb w3, [x0], #1
    cmp x3, #10         // b'\n' == 10
    b.eq start_distance
    cmp x3, #32         // b' ' == 32
    b.eq parse_time
    sub x3, x3, #48
    mul x1, x1, x10
    add x1, x1, x3
    b parse_time
start_distance:
    add x0, x0, #9      // "Distance:"
parse_distance:
    ldrb w3, [x0], #1
    cmp x3, #10         // b'\n' == 10
    b.eq calculate
    cmp x3, #32         // b' ' == 32
    b.eq parse_distance
    sub x3, x3, #48
    mul x2, x2, x10
    add x2, x2, x3
    b parse_distance
calculate:
    mov x3, x1          // these are nonsensical moves to let me reuse code
    mov x4, x2
    // x3 contains time, x4 contains distance
    mov x8, x3          // time
    mov x9, x4          // distance
    ucvtf d3, x3        // time
    ucvtf d4, x4        // distance
    mov x5, #2
    ucvtf d5, x5        // 2
    mov x6, #4
    ucvtf d6, x6        // 4
    fmul d7, d3, d3     // t**2
    fmul d4, d4, d6     // 4d
    fsub d7, d7, d4     // t**2 - 4d
    fsqrt d7, d7        // sqrt(t**2 - 4d)
    fdiv d7, d7, d5     // sqrt(t**2) / 2
    fdiv d3, d3, d5     // t / 2
    fadd d4, d3, d7     // larger zero
    fsub d3, d3, d7     // smaller zero
    fcvtmu x4, d4       // rounding down
    fcvtpu x3, d3       // rounding up
    // check math
    sub x5, x8, x3      // (t - b)
    mul x5, x5, x3      // (t - b) * b
    cmp x5, x9          // this was an exact solution
    sub x4, x4, x3
    add x4, x4, #1      // number of solutions
    b.ne finish            // exact solution
    sub x4, x4, #2
finish:
    mov x0, x4
    ret
