# Syscalls
All syscalls will be for Linux, i will be using a table for [the Linux 4 kernel](https://chromium.googlesource.com/chromiumos/docs/+/master/constants/syscalls.md).
all System calls are x86_64, 32 bit programs are outdated and supporting them isn't my utmost priority.

## File Descriptors
0.  STDIN
1.  STDOUT
2.  STDERR

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

### exit 0x3C(60) ((u8 int rdi) error_code)
[Linux Manual](https://man7.org/linux/man-pages/man2/exit.2.html)

Terminates the calling process, you need it otherwise the process will result in a segfault.
The reason why the type in quecto is u8 and not i32 is because linux will just wrap it around from 0-255 anyways so why waste the extra bytes.

Assembly:
```x86_64
mov rax, 60
mov rdi, <error_code>
syscall
```

Quecto:
```
exit(<error_code>)
```
or
```
fn:u8 start()
{
    return <error_code>;
}
```
since start already contains an implicit exit()

### write 0x01(1) ((i32 int rdi) fd)

Assembly:
```x86_64
section .rodata
msg db 'Hello, World'

section .text
global _start

_start:
    ; Example Begin
    mov rax, 0x01 ; write
    mov rdi, 0x01 ; STDOUT
    mov rsi, msg
    mov rdx, 12 ; Length of message
    syscall
    ; Example End

    mov rax, 0x3C ; Exit
    mov rdi, 1
    syscall
```

Quecto (SUBJECT TO CHANGE):
```
fn:u8 start()
{
    let msg: str = "Hello, World"; 
    sys_write(STDOUT, &msg, str_len(msg))
}
```