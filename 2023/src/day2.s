      .global    day2_p1
      .type      day2_p1, "function"

      .p2align   4

// should I be using r or x?
// are there certain registers that I should be preferring?
// how fast/slow are comparison against literals vs constants vs registers?
// should I just be defining these in their own files?
day2_p1:

    add x1, x1, x0      // final memory address
    mov x2, #0          // total = 0
    mov x10, #10

game_loop:
    cmp x0, x1
    b.eq finish
    add x0, x0, #5      // Skip past "Game "
    ldrb w3, [x0], #1
    sub x3, x3, #48     // subtract ASCII digit
game_id_parse_loop:
    ldrb w4, [x0], #1
    cmp x4, #58        // b':' == 58
    b.eq draw_loop
    mul x3, x3, x10     // id = id * 10
    sub x4, x4, #48     // b'0' == 48
    add x3, x3, x4      // id += next_digit
    b game_id_parse_loop

draw_loop:
    add x0, x0, #1      // skip past the empty space
    ldrb w4, [x0], #1   // read in a number
    sub x4, x4, #48     // b'0' == 48
cube_parse_loop:
    ldrb w5, [x0], #1
    cmp x5, #32         // b' ' == 32
    b.eq parse_color
    sub x5, x5, #48     // next_digit to int
    mul x4, x4, x10     // cube = cube * 10
    add x4, x4, x5      // cube = cube + next_digit
    b cube_parse_loop
    // set initial variable
parse_color:
    ldrb w5, [x0], #1
    cmp x5, #114        // b'r' == 114
    b.eq red_check
    cmp x5, #103        // b'g' == 103
    b.eq green_check
// don't move this, I'm relying on the no-branchy
blue_check:
    add x0, x0, 3
    cmp x4, #14
    b.gt impossible_game
    b finish_hand
red_check:
    add x0, x0, 2
    cmp x4, #12
    b.gt impossible_game
    b finish_hand
green_check:
    add x0, x0, #4
    cmp x4, #13
    b.gt impossible_game
    b finish_hand
finish_hand:
    ldrb w4, [x0], #1       // comma, semi-colon, or newline
    cmp x4, #10             // b'\n' == 10
    b.ne draw_loop
    add x2, x2, x3          // total = total + game_id
    b game_loop

impossible_game:            // skip to the end of the game
    ldrb w4, [x0], #1
    cmp x4, #10             // b'\n' == 10
    b.ne impossible_game
    b game_loop

finish:
    mov x0, x2
    ret












      .global    day2_p2
      .type      day2_p2, "function"
day2_p2:

    add x1, x1, x0      // final memory address
    mov x2, #0          // total = 0
    mov x10, #10

game_loop_2:
    cmp x0, x1
    b.eq finish
    mov x6, #0          // r_max
    mov x7, #0          // g_max
    mov x8, #0          // b_max
    add x0, x0, #5      // Skip past "Game "
game_id_parse_loop_2:
    ldrb w3, [x0], #1
    cmp x3, #58        // b':' == 58
    b.ne game_id_parse_loop_2
draw_loop_2:
    add x0, x0, #1      // skip past the empty space
    ldrb w4, [x0], #1   // read in a number
    sub x4, x4, #48     // b'0' == 48
cube_parse_loop_2:
    ldrb w5, [x0], #1
    cmp x5, #32         // b' ' == 32
    b.eq parse_color_2
    sub x5, x5, #48     // next_digit to int
    mul x4, x4, x10     // cube = cube * 10
    add x4, x4, x5      // cube = cube + next_digit
    b cube_parse_loop_2
    // set initial variable
parse_color_2:
    ldrb w5, [x0], #1
    cmp x5, #114        // b'r' == 114
    b.eq red_check_2
    cmp x5, #103        // b'g' == 103
    b.eq green_check_2
// don't move this, I'm relying on the no-branchy
blue_check_2:
    add x0, x0, 3
    cmp x4, x8
    b.le finish_hand_2
    mov x8, x4
    b finish_hand_2
red_check_2:
    add x0, x0, 2
    cmp x4, x6
    b.le finish_hand_2
    mov x6, x4
    b finish_hand_2
green_check_2:
    add x0, x0, #4
    cmp x4, x7
    b.le finish_hand_2
    mov x7, x4
    b finish_hand_2
finish_hand_2:
    ldrb w4, [x0], #1       // comma, semi-colon, or newline
    cmp x4, #10             // b'\n' == 10
    b.ne draw_loop_2
    mul x6, x6, x7
    mul x6, x6, x8
    add x2, x2, x6          // total = total + power
    b game_loop_2

finish_2:
    mov x0, x2
    ret
