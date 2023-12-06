```
gdb --args target/debug/deps/aoc2023-b86eedb9bba114e8 day1
```
This will pass the arguments day1 argument to gdb


The thread stuff is maybe complicating things? But I tried just running it as a boring binary, and I think lots of the debug information get's erased/isn't present with the global asm macro.

https://rust-lang.github.io/rfcs/1548-global-asm.html
> The current way of including external assembly is to compile the assembly files using gcc in build.rs and link them into the Rust program as a static library.

It might be worth looking into this if it will give my pretty debug information. I suspect that GDB is going to be brutally necessary on this little quest.