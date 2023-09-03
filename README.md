# Quecto
!!! USE RUST NIGHTLY !!! (just for cargo +nightly fmt)

I have already tried building a language in typescript.
Now I am going to make another one, compiled this time, in Rust.
By Reading the checklist you can probably see the amount of tests that will be implemented,
This should be fairly easy thanks to rusts test system. The reason why there are so many
tests is because i want to know when something breaks before i start implementing new 
features.

## Other README's
Read them before reading the rest of the document, listed in correct order
1. [Syntax Document](quecto.md)
2. [System Call's In Quecto](syscalls.md)
3. [Quecto Example Programs](example_programs.md)

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
  - [ ] Scope - {}
  - [ ] Functions
    - [ ] Declaration
    - [ ] Calling
  - [ ] Control Flow
    - [ ] If Statements
    - [ ] Loops
    - [ ] While Loops
    - [ ] For Loops
- [ ] Compiler
  - [ ] x86_64 Representation in rust
  - [ ] System Call Representation in rust
    - [ ] Linux
  - [ ] Return statement
    - [ ] Tests
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
  - [ ] Function calls
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