Problem: the memory in the radix is not zerod. The first increment is not writing 0. AHHH

MAX NUM was successfully 9, I wonder if I have an off by 1 error?

I am probably reverse engineering to the real number incorrectly

That will be the next thing to debug, around line 169. This is the part responsible for walking forward until I found a slot that I have previously used.
