use std::fmt::Display;

// TODO: Add 4 byte, 2 byte and singular byte registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register64
{
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

macro_rules! reg_display {
    ($match_against:ident $($name:ident $to:ident),*) => {
        match $match_against
        {
           $(
            Register64::$name => stringify!($to),
           )*
        }
    };
}

impl Display for Register64
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let str = reg_display!(self
            Rax rax,
            Rcx rcx,
            Rdx rdx,
            Rbx rbx,
            Rsi rsi,
            Rdi rdi,
            Rsp rsp,
            Rbp rbp,
            R8 r8,
            R9 r9,
            R10 r10,
            R11 r11,
            R12 r12,
            R13 r13,
            R14 r14,
            R15 r15
        );

        write!(f, "{str}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmValue
{
    Reg(Register64),
    MemLoc(usize),
    MemReg(Register64),
    MemRegOffset(Register64, isize),
    Imm(u64)
}

impl Display for AsmValue
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let out = match self
        {
            AsmValue::Reg(reg) => reg.to_string(),
            AsmValue::MemLoc(loc) => format!("[{loc}]"),
            AsmValue::MemReg(reg) => format!("[{reg}]"),
            AsmValue::MemRegOffset(reg, off) => format!("[{reg} {} {off}]", if *off < 0 {"-"} else {"+"}),
            AsmValue::Imm(imm) => imm.to_string(),
        };

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Asmx86_64
{
    Mov(AsmValue, AsmValue),
    Syscall,
    Label(String),
    Global(String),
    Section(String),
    Call(String),
    Return,
    Push(AsmValue),
    Pop(AsmValue),
    Sub(AsmValue, AsmValue),
    Add(AsmValue, AsmValue)
}

impl ToString for Asmx86_64
{
    fn to_string(&self) -> String
    {
        match self
        {
            Asmx86_64::Mov(dest, src) => String::from(format!("mov {dest}, {src}")),
            Asmx86_64::Syscall => String::from("syscall"),
            Asmx86_64::Label(s) => String::from(format!("{s}:")),
            Asmx86_64::Global(s) => String::from(format!("global {s}")),
            Asmx86_64::Section(s) => String::from(format!("section {s}")),
            Asmx86_64::Call(s) => String::from(format!("call {s}")),
            Asmx86_64::Return => String::from("ret"),
            Asmx86_64::Push(s) => String::from(format!("push {s}")),
            Asmx86_64::Pop(d) => String::from(format!("pop {d}")),
            Asmx86_64::Sub(d, s) => String::from(format!("sub {d}, {s}")),
            Asmx86_64::Add(d, s) => String::from(format!("sub {d}, {s}")),
        }
    }
}