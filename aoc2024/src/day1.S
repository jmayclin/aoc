
#define PARSED_NUMBERS_START x2     // the start of the parsed u32s
#define PARSED_NUMBER_POINTER x3    // the pointer to the current u32, store here
#define PARSED_NUMBERS_COUNT x6     // the total number of u32s that have been stored

#define END_OF_INPUT x7

#define MAX_NUM x8
#define RADIX_START x9
#define RADIX_CURRENT x10

#define PARSING_SCRATCH x4
#define PRASING_ACCUMULATOR x5

#define ASCII_0 #48
#define ASCII_SPACE #32
#define ASCII_NEWLINE #10

.global    day1_p1
.type      day1_p1, "function"

.p2align   4

day1_p1:
    add END_OF_INPUT, x0, x1
    sub sp, sp, #16             // why am I doing this, something with accidentally clobering return registers?
    mov PARSED_NUMBERS_START, sp
    mov PARSED_NUMBER_POINTER, PARSED_NUMBERS_START
    mov PARSED_NUMBERS_COUNT, #0

parse_line_start:
    cmp x0, END_OF_INPUT
    b.eq find_max_start

    mov x5, #0
parse_line_left:
    ldrb w4, [x0], #1   // load x0 into register x2, then r0 += 1

    cmp x4, ASCII_SPACE
    b.eq parse_line_middle

    sub x4, x4, ASCII_0     // b'0' -> 48
    mov x10, #10
    mul x5, x5, x10
    add x5, x5, x4
    b parse_line_left

parse_line_middle:
    // load the number into the parsed stuff
    add x0, x0, #2  // skip the spaces
    str w5, [PARSED_NUMBER_POINTER]
    add PARSED_NUMBERS_COUNT, PARSED_NUMBERS_COUNT, #1
    sub PARSED_NUMBER_POINTER, PARSED_NUMBER_POINTER, #32

    mov x5, #0
parse_line_right:
    ldrb w4, [x0], #1

    cmp x4, ASCII_NEWLINE
    b.eq parse_line_finish

    sub x4, x4, ASCII_0
    mul x5, x5, x10
    add x5, x5, x4
    b parse_line_right
parse_line_finish:
    str w5, [PARSED_NUMBER_POINTER]
    add PARSED_NUMBERS_COUNT, PARSED_NUMBERS_COUNT, #1
    sub PARSED_NUMBER_POINTER, PARSED_NUMBER_POINTER, #32
    b parse_line_start

find_max_start:
    mov x0, PARSED_NUMBERS_START

    mov x5, #0
find_max_iter:
    cmp x0, PARSED_NUMBER_POINTER
    mov MAX_NUM, x5
    b.eq zero_stack_setup

    ldr w4, [x0]
    sub x0, x0, #32
    cmp x4, x5
    b.lt find_max_iter
    mov x5, x4
    b find_max_iter

zero_stack_setup:
    mov RADIX_START, PARSED_NUMBER_POINTER
    mov x0, PARSED_NUMBER_POINTER

    mov x10, MAX_NUM
    lsl x10, x10, #1
    sub x5, RADIX_START, x10
    mov x3, #0

zero_stack_iter:
    cmp x0, x5
    b.eq assemble_radix_setup

    str w3, [x0]
    sub x0, x0, #32
    b zero_stack_iter

    // add "number" to RADIX start to get address of counter slot.
    // add +1 for right byte
assemble_radix_setup:
    mov x0, PARSED_NUMBERS_START

assmeble_radix_iter:
    cmp x0, x5
    b.eq subtraction_setup

    ldr w4, [x0]                // load the u32 from parsed_numbers
    sub x0, x0, #32             // increment the parsed_numbers pointer
    lsl x4, x4, #1              // multiple the number by 2
    sub x4, RADIX_START, x4     // find the address to store at

    ldrb w10, [x4]                // read from the address
    add x10, x10, #1              // increment by 1
    strb w10, [x4]                // store to the address

    ldr w4, [x0]                // load the u32 from parsed_numbers
    sub x0, x0, #32             // increment the parsed_numbers pointer
    lsl x4, x4, #1              // multiple the number by 2
    add x4, x4, #1              // add one to account for the "right" column
    sub x4, RADIX_START, x4     // find the address to store at

    ldrb w10, [x4]                // read from the address
    add x10, x10, #1              // increment by 1
    strb w10, [x4]                // store to the address

    b assmeble_radix_iter

subtraction_setup:
    mov x0, RADIX_START           // left pointer
    sub x1, RADIX_START, #1       // right pointer

    lsl x11, MAX_NUM, #1
    sub x10, RADIX_START, x11

    mov x8, #0

    ldrb w2, [x0]
    sub x0, x0, #2

    ldrb w3, [x1]
    sub x1, x1, #2

// check for end
subtraction_iter:
    cmp x0, x10
    b.eq finish

advance_left_iter:
    cmp x2, #0
    b.ne advance_right_iter
    ldrb w2, [x0]
    sub x0, x0, #2

advance_right_iter:
    cmp x3, #0
    b.ne compare
    ldrb w3, [x1]
    sub x1, x1, #2

compare:
    sub x4, RADIX_START, x0
    add x4, x4, #2
    lsr x4, x4, #1

    sub x5, RADIX_START, x1
    add x5, x5, #3                 // 2 for the extra shift, and 1 for the right offset
    lsr x5, x5, #1

    sub x6, x5, x4
    //abs x6, x6

    add x8, x8, x6
finish:
    mov x0, x8
    ret



