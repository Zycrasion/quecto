# Syscalls
All syscalls will be for Linux, i will be using a table for [the Linux 4 kernel](https://chromium.googlesource.com/chromiumos/docs/+/master/constants/syscalls.md).
all System calls are x86_64, 32 bit programs are outdated and supporting them isn't my utmost priority.

## Quecto System Calls
Listed in order of implementation priority.
Titles follow this convention.
```
name <hex_code>(<base 10 form>) (params)
```
where params follow
```
(quecto_type c_type register_name) param_name
```

### exit 0x3C(60) ((u8 int rsi) error_code)
[Linux Manual](https://man7.org/linux/man-pages/man2/exit.2.html)

Terminates the calling process, you need it otherwise the process will result in a segfault.

Assembly:
```x86_64
mov rax, 60
mov rsi, <error_code>
syscall
```