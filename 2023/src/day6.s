    .global day6_p1
    .type day6_p1, "function"
    .p2align 4

// hmm, I think it's really about time I learned
// how to call a function. I don't think it's fair to
// call this "idomatic" assembly. It is very ugly
day6_p1:
    mov x10, #10
    mov x11, #1         // accumulator
    add x1, x1, x0
    add x0, x0, #5      // "Time:"
    mov x2, x0          // pointer to second row
find_second_line:
    ldrb w3, [x2], #1
    cmp x3, #10         // b'\n' == 10
    b.ne find_second_line
    mov x1, x2          // store the end of the first line
    sub x1, x1, #1
    add x2, x2, #9      // "Distance:"
col_loop:
    mov x3, #0          // time
    mov x4, #0          // distance
first_space:
    ldrb w5, [x0], #1
    cmp x5, #68         // b'D' == 68, if 'd', then finish
    b.eq finish
    cmp x5, #32         // b' ' == 32
    b.eq first_space
time_parse:
    sub x5, x5, #48         // b'0' == 48
    mul x3, x3, x10
    add x3, x3, x5
    ldrb w5, [x0], #1
    cmp x5, #48         // either space or newline, both less than 48
    b.ge time_parse
second_space:
    ldrb w5, [x2], #1
    cmp x5, #32         // b' ' == 32
    b.eq second_space
distance_parse:
    sub x5, x5, #48         // b'0' == 48
    mul x4, x4, x10
    add x4, x4, x5
    ldrb w5, [x2], #1
    cmp x5, #48         // either space or newline, both less than 48
    b.ge distance_parse
    // x3 contains time, x4 contains distance
    mov x8, x3          // time
    mov x9, x4          // distance
    ucvtf s3, x3        // time
    ucvtf s4, x4        // distance
    mov x5, #2
    ucvtf s5, x5        // 2
    mov x6, #4
    ucvtf s6, x6        // 4
    fmul s7, s3, s3     // t**2
    fmul s4, s4, s6     // 4d
    fsub s7, s7, s4     // t**2 - 4d
    fsqrt s7, s7        // sqrt(t**2 - 4d)
    fdiv s7, s7, s5     // sqrt(t**2) / 2
    fdiv s3, s3, s5     // t / 2
    fadd s4, s3, s7     // larger zero
    fsub s3, s3, s7     // smaller zero
    fcvtmu x4, s4       // rounding down
    fcvtpu x3, s3       // rounding up
    // check math
    sub x5, x8, x3      // (t - b)
    mul x5, x5, x3      // (t - b) * b
    cmp x5, x9          // this was an exact solution
    sub x4, x4, x3
    add x4, x4, #1      // number of solutions
    b.ne finish_math            // exact solution
    sub x4, x4, #2
finish_math:
    mul x11, x11, x4
    b col_loop
finish:
    mov x0, x11
    ret
