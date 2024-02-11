```
gdb --args target/debug/deps/aoc2023-b86eedb9bba114e8 day1
```
This will pass the arguments day1 argument to gdb

# My Assembly Learnings
Rust does have inline assembly, but I don't think it includes debug information? Which given my lowly human nature, is a requirement for me.

# Assembly Build Approach



# Nasty Liar
This article is at best, unhelpful, and at worst, a liar
https://community.arm.com/arm-community-blogs/b/architectures-and-processors-blog/posts/how-to-call-a-function-from-arm-assembler
```
  cargo:warning=src/day8.s: Assembler messages:

  cargo:warning=src/day8.s:7: Error: unknown mnemonic `push' -- `push {ip,lr}'

  cargo:warning=src/day8.s:25: Error: unknown mnemonic `pop' -- `pop {ip,pc}'

  exit status: 1
```

# How to call other functions
The article above told me I should use push/pop, but the compiler started swearing at me when I tried that, so that doesn't seem to be the move.

What does Godbolt say?
```
        sub     sp, sp, #32
        stp     x29, x30, [sp, #16]             // 16-byte Folded Spill
        add     x29, sp, #16
        stur    wzr, [x29, #-4]
```

It doesn't seem to be using pop or push. What is the 16 byte folded reload? I wonder if the article above is using a tool that "compiles" the pop/push statements into the assembly that clang is now emitting?

Nope
> armv8 dropped push and pop, compilers often use stp and ldp instead. Example at: https://github.com/cirosantilli/arm-assembly-cheat/blob/f8d78775bd052e9ead579a408c0a2a1651adb9f0/v8/common_arch.h#L20
-https://stackoverflow.com/questions/27095099/push-and-pop-in-arm
Thank you internet person.

I still don't entirely understand the different ARM versions. But I'm assuming that _most_ of the stuff stays relatively similar across platforms? I'm content to leave that stone unturned for the purpose of Advent of Code stuff.

https://learn.arm.com/learning-paths/servers-and-cloud-computing/exploiting-stack-buffer-overflow-aarch64/frame-layout/
This seems like a nice article

Ooooh, `stp` stands for "store pair", and will store both x30 and x29
x30 -> branch register, which instruction/address should we go to when executing a `ret` statement?
x29 -> frame pointer
> Register x29 contains the “ frame pointer ”, and is sometimes called fp. The frame pointer points to a location in the stack frame that contains the “frame record”. The frame record has two fields: a pointer to the frame record of the function that called the current function; and the address the current function needs to return to.

This explains why my willy nilly previous things didn't destroy the stack. Since I didn't touch x29 and x30 I didn't corrupt anything and it was able to neatly return to the rust program space. And the 16 stack pointer thing is because of the 2 8 bytes pointers that get stored.



The thread stuff is maybe complicating things? But I tried just running it as a boring binary, and I think lots of the debug information get's erased/isn't present with the global asm macro.

https://rust-lang.github.io/rfcs/1548-global-asm.html
> The current way of including external assembly is to compile the assembly files using gcc in build.rs and link them into the Rust program as a static library.

It might be worth looking into this if it will give my pretty debug information. I suspect that GDB is going to be brutally necessary on this little quest.
