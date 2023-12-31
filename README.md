# Quecto
!!! USE RUST NIGHTLY !!! (just for cargo +nightly fmt)

I have already tried building a language in typescript.
Now I am going to make another one, compiled this time, in Rust.
By Reading the checklist you can probably see the amount of tests that will be implemented,
This should be fairly easy thanks to rusts test system. The reason why there are so many
tests is because i want to know when something breaks before i start implementing new 
features.

## What's happening right now?
I'm replacing the parser / tokeniser into it's own pipeline managed by a seperate crate. I will be developing this crate.

## Name
It's a metric prefix, representing 10 to the power of -30.

## Dependencies
Nasm atleast version 2.14.
```bash
sudo apt install nasm
```

## Compiling
This has been tested on WSL.
### Predefined programs
```bash
cargo run -- --file quecto/test.quecto --output quecto/build/test
./quecto/build/test
```

### Testing
There should be many tests for the program already.
just run
```bash
cargo test
```

## Other README's
Read them before reading the rest of the document, listed in correct order
1. [Syntax Document](quecto.md)
2. [Quecto Example Programs](example_programs.md)
3. [ABI](ABI.md)
4. [System Calls In Quecto](syscalls.md)

## Files Working
- [simplest_program.quecto](quecto/simplest_program.quecto)
- [test.quecto](quecto/test.quecto)

## Checklist:
- [x] Basic Syntax document [quecto.md](quecto.md)
- [x] Tokenising
- [ ] Abstract Syntax Tree Building (AST)
  - [x] Return Statement - return u8
  - [x] Values
    - [x] Numbers
    - [x] Booleans
    - [x] Characters
    - [x] Strings
  - [ ] Maths
  - [ ] Variables
    - [ ] Declaration
    - [ ] Referencing
  - [x] Scope - {}
  - [ ] Functions
    - [x] Declaration
    - [x] Calling
    - [ ] Parameters
  - [ ] Control Flow
    - [ ] If Statements
    - [ ] Loops
    - [ ] While Loops
    - [ ] For Loops
- [ ] Compiler
  - [x] x86_64 Representation in rust
  - [x] System Call Representation in rust
    - [x] Linux
  - [x] Return statement
    - [ ] Tests
  - [ ] Proper Types ( Current Type System is an illusion )
  - [ ] Maths
    - [ ] Tests
  - [ ] Variables (At the same time as doing maths)
    - [ ] Types
    - [ ] Tests
  - [ ] Control Flow
    - [ ] If Statements
      - [ ] Tests
    - [ ] Loops
      - [ ] Tests
    - [ ] While Loops
      - [ ] Tests
    - [ ] For Loops
      - [ ] Tests
  - [x] Function calls
    - [ ] I/O Functions
      - [ ] Tests
    - [ ] Some basic system calls implemented in the language
      - [ ] Tests
  - [ ] Function Definitions
    - [ ] Tests
    - [ ] System-V ABI
      - [ ] Tests
  - [ ] Other Features i haven't thought of yet
- [ ] Good CLI
  - [ ] Better Help Menu
  - [ ] Flags to help debug programs
    - [ ] Outputting debug symbols?
- [ ] Language Standard Library / Other Language Features
  - [ ] Way to get Command Line Arguments
  - [ ] Arrays
  - [ ] Heap Allocation / Freeing
  - [ ] Vector (LinkedList?)
  - [ ] String, built on top of Vector
  - [ ] Structures?
  - [ ] Unit Tests
- [ ] Meshing External Libraries With Quecto Binaries