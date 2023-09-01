# Quecto Design

## Anatomy of Quecto Program
The entry point of a Quecto program will be start
```
fn:u8 start()
{
    return 0;
}
```

## Variables in Quecto
Variables in quecto will be very similar to rust syntax, however, all types MUST be explicit.
```
let unsigned_8_bit_value : u8 = 255;
let signed_16_bit_value : i16 = 32_000;
let string_variable : str = "Hello";
```
### Available Types:
Unsigned:
-   u8
-   u16
-   u32
-   u64

Signed
-   i8
-   i16
-   i32
-   i64

Float:
-   f32
-   f64

Other:
-   bool
-   char
-   str

## Maths
Very simple, just like any other language.
```
let result : i32 = 5 * 5 + 2
```
which is 27, following operator precedence.

## Function calls
Just like any other language
```
abc(123)
```

## Control Flow
### If statements
```
let i : i32  = 1;
let b : bool = i < 2; // true
if b || i > 3
{
    log("wow")
}
```
### Loops
Most basic loop:
```
loop
{
    break;
}
```
While loop:
```
loop while i == true
{

}
```
For loop:
```
loop for i in 0..10
{
    continue;
}
```

## I/O in Quecto
Aimed to be as simple as possible. While still providing options for behaviour to be expanded.
```
fn:u8 start()
{
    io.start()
    let input : String = io.readline();
    let input_single_char : char = io.read(1);
    io.log(123)
    return 0;
}
```
### What is io.start()
Just in case people want to handle I/O differently i don't want the underlying langauge library to get in the way of that, so all of quecto's I/O initialisation code will go into `io.start`, this also means that if you don't include it into your program, it won't include the code, making the binary smaller.

## External Quecto Files
Will be aimed to be as simple as possible.
Probably something like:

start.q
```
import wow

fn:u8 start()
{
    io.start();
    wow.wow();
    return 0;
}
```

wow.q
```
pub fn:void wow()
{
    io.log("wow");
}

fn:void not_cool()
{
    io.log("i cant get called due to no public visibility modifier");
}
```

## Defining Your Own Types
I don't know yet, very advanced, not gonna think about it for the time being.