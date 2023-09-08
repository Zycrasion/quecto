use std::fmt::Display;

use super::SupportedSystems;

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

impl Display for Register64
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let str = match self
        {
            Register64::Rax => "rax",
            Register64::Rcx => "rcx",
            Register64::Rdx => "rdx",
            Register64::Rbx => "rbx",
            Register64::Rsi => "rsi",
            Register64::Rdi => "rdi",
            Register64::Rsp => "rsp",
            Register64::Rbp => "rbp",
            Register64::R8 => "r8",
            Register64::R9 => "r9",
            Register64::R10 => "r10",
            Register64::R11 => "r11",
            Register64::R12 => "r12",
            Register64::R13 => "r13",
            Register64::R14 => "r14",
            Register64::R15 => "r15",
        };

        write!(f, "{str}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination64
{
    Reg(Register64),
    MemLoc(usize),
    MemReg(Register64),
    MemRegOffset(Register64, usize),
}

impl Display for Destination64
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let out = match self
        {
            Destination64::Reg(reg) => reg.to_string(),
            Destination64::MemLoc(loc) => format!("[{loc}]"),
            Destination64::MemReg(reg) => format!("[{reg}]"),
            Destination64::MemRegOffset(reg, off) => format!("[{reg} + {off}]"),
        };

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source64
{
    Reg(Register64),
    MemLoc(usize),
    MemReg(Register64),
    MemRegOffset(Register64, usize),
    Imm(u64),
}

impl Display for Source64
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let out = match self
        {
            Source64::Reg(reg) => reg.to_string(),
            Source64::MemLoc(loc) => format!("[{loc}]"),
            Source64::MemReg(reg) => format!("[{reg}]"),
            Source64::MemRegOffset(reg, off) => format!("[{reg} + {off}]"),
            Source64::Imm(imm) => format!("{imm}"),
        };

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Asmx86_64
{
    Mov(Destination64, Source64),
    Syscall,
    Label(String),
    Global(String),
    Section(String),
    Call(String),
    Return,
    Push(Source64),
    Pop(Destination64),
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
        }
    }
}