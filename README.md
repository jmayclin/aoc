```
gdb --args target/debug/deps/aoc2023-b86eedb9bba114e8 day1
```
This will pass the arguments day1 argument to gdb

# To Learn
- [ ] I want to have multiple globals (functions?) in a single assembly file
- [ ] What are all of the various fields in the assembly header things?
- [ ] When is `x8` used?
- [ ] What are the performance differences between `b` and `bl`?
- [ ] Why can't I directly subtract the `sp` register?
- [ ] Is `sp` a register? 


# My Assembly Learnings
Rust does have inline assembly, but I don't think it includes debug information? Which given my lowly human nature, is a requirement for me.


# Nasty Liar
This seems to be lying to me
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

This explains why my willy nilly previous things didn't destroy the stack. Since I didn't touch x29 and x30 I didn't corrupt anything and it was able to neatly return to the rust program space. And the 16 stack pointer thing is because of the 2 8 bytes pointers that get stored. There's something appealing about how delightfully manual that is.

`ulimit -s` <- I think this tells me the maximum size for a stack? So I don't think that will be an option for my nightmarish random access thing that I have planned for day 8.

Oh wait, I think that is in Kilobytes? 
```
ubuntu@ip-172-31-4-49:~/workspace/aoc2023$ ulimit -a
real-time non-blocking time  (microseconds, -R) unlimited
core file size              (blocks, -c) 0
data seg size               (kbytes, -d) unlimited
scheduling priority                 (-e) 0
file size                   (blocks, -f) unlimited
pending signals                     (-i) 15206
max locked memory           (kbytes, -l) 487972
max memory size             (kbytes, -m) unlimited
open files                          (-n) 1048576
pipe size                (512 bytes, -p) 8
POSIX message queues         (bytes, -q) 819200
real-time priority                  (-r) 0
stack size                  (kbytes, -s) 8192
cpu time                   (seconds, -t) unlimited
max user processes                  (-u) 15206
virtual memory              (kbytes, -v) unlimited
file locks                          (-x) unlimited
```

hehe. Then yes, I think I can skip using mmap. This should also make things much speedier 🏃. I should probably still do the "dynamic" mmap based route just so that I can actually do the function cally thing.

https://developer.arm.com/documentation/den0024/a/The-ABI-for-ARM-64-bit-Architecture/Register-use-in-the-AArch64-Procedure-Call-Standard/Parameters-in-general-purpose-registers?lang=en
This tells me which registers I have to save.
> Argument registers (X0-X7)
> Caller-saved temporary registers (X9-X15)
> Callee-saved registers (X19-X29)
> Registers with a special purpose (X8, X16-X18, X29, X30)

So if I put something in X19-X29, it will still be there when I get back. But I think this means that I would have to store anything that's already there. Which is irritating 😠

It seems like caller vs callee saved are both approximately the same amount of work if you don't know who is calling your code and you don't know that code that you are calling. But if you know the code that you are calling, then you can spill over into X9-X15 without consequence? Fun! And Also X0-X7. Still confused on the role of X8 tho. 

Oh fun, there appears to be constant registers? https://stackoverflow.com/questions/42788696/why-might-one-use-the-xzr-register-instead-of-the-literal-0-on-armv8

Hmm, the OS is getting angry at me when I am trying to do stacky things.











The thread stuff is maybe complicating things? But I tried just running it as a boring binary, and I think lots of the debug information get's erased/isn't present with the global asm macro.

https://rust-lang.github.io/rfcs/1548-global-asm.html
> The current way of including external assembly is to compile the assembly files using gcc in build.rs and link them into the Rust program as a static library.

It might be worth looking into this if it will give my pretty debug information. I suspect that GDB is going to be brutally necessary on this little quest.
