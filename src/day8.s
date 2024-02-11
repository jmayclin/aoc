    .global day8_p1
    .type day8_p1, "function"
    .p2align 4

//
day8_p1:
    push {ip, lr}
    add x1, x0, x1          // ptr to end
    mov x2, x0              // char ptr, count how many entries
    mov x4, #0              // accumulator
count_lines:
    cmp x2, x1
    b.eq allocate
    ldrb w3, [x2], #1
    cmp x3, #10             // b'\n' == 10
    b.ne count_lines
    add x4, x4, #1
    b count_lines
allocate:
    sub x4, x4, #2          // for the leading lines
    mov x4, x4, lsl #4
    mov x0, x4
    bl malloc
    cbz x0, allocation_failed  // Check if malloc returned NULL
    pop {ip, pc}
    ret
    //mov x0, x4

allocation_failed:
    mov x0, #-1
    ret

    // mov x0, #0       // Set x0 to 0 for brk
    // ldr x1, =4096     // Set x1 to the size of memory to allocate (e.g., 4096 bytes)
    // mov x8, #214      // System call number for brk on AArch64 (syscall: 0xD6)


