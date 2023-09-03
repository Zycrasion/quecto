# Quecto ABI
Quecto uses the [System-V ABI](https://wiki.osdev.org/System_V_ABI), Specifically the x86_64 version.

## Overview
Parameters:
1.  rdi
2.  rsi
3.  rdx
4.  rcx
5.  r8
6.  r9

And further values are placed on the stack in reverse order.
The parameters on the stack may be modified by the called function.

Preserved Registers:
1.  rbx
2.  rsp
3.  rbp
4.  r12
5.  r13
6.  r14
7.  r15

Scratch Registers:
1.  rax
2.  rdi
3.  rsi
4.  rdx
5.  rcx
6.  r8
7.  r9
8.  r10
9.  r11

Return values go into the `rax` register and if the return value is 128-bit, the upper 64 bits are placed onto rdx.
