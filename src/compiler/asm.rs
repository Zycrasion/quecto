use std::fmt::Display;

use crate::shared::types::QuectoNumberTypes;

// TODO: Add 4 byte, 2 byte and singular byte registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register
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

impl Display for Register
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let str = match self
        {
            Register::Rax => "rax",
            Register::Rcx => "rcx",
            Register::Rdx => "rdx",
            Register::Rbx => "rbx",
            Register::Rsi => "rsi",
            Register::Rdi => "rdi",
            Register::Rsp => "rsp",
            Register::Rbp => "rbp",
            Register::R8 => "r8",
            Register::R9 => "r9",
            Register::R10 => "r10",
            Register::R11 => "r11",
            Register::R12 => "r12",
            Register::R13 => "r13",
            Register::R14 => "r14",
            Register::R15 => "r15",
        };

        write!(f, "{str}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Destination
{
    Reg(Register),
    MemLoc(usize),
    MemReg(Register),
    MemRegOffset(Register, usize),
}

impl Display for Destination
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let out = match self
        {
            Destination::Reg(reg) => reg.to_string(),
            Destination::MemLoc(loc) => format!("[{loc}]"),
            Destination::MemReg(reg) => format!("[{reg}]"),
            Destination::MemRegOffset(reg, off) => format!("[{reg} + {off}]"),
        };

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source
{
    Reg(Register),
    MemLoc(usize),
    MemReg(Register),
    MemRegOffset(Register, usize),
    Imm(QuectoNumberTypes),
}

impl Display for Source
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let out = match self
        {
            Source::Reg(reg) => reg.to_string(),
            Source::MemLoc(loc) => format!("[{loc}]"),
            Source::MemReg(reg) => format!("[{reg}]"),
            Source::MemRegOffset(reg, off) => format!("[{reg} + {off}]"),
            Source::Imm(imm) => format!("{imm}"),
        };

        write!(f, "{out}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Assembly
{
    Mov(Destination, Source),
    Syscall,
    Label(String),
    Global(String),
    Section(String),
    Call(String),
    Return,
    Push(Source),
    Pop(Destination),
}

impl ToString for Assembly
{
    fn to_string(&self) -> String
    {
        match self
        {
            Assembly::Mov(dest, src) => String::from(format!("mov {dest}, {src}")),
            Assembly::Syscall => String::from("syscall"),
            Assembly::Label(s) => String::from(format!("{s}:")),
            Assembly::Global(s) => String::from(format!("global {s}")),
            Assembly::Section(s) => String::from(format!("section {s}")),
            Assembly::Call(s) => String::from(format!("call {s}")),
            Assembly::Return => String::from("ret"),
            Assembly::Push(s) => String::from(format!("push {s}")),
            Assembly::Pop(d) => String::from(format!("pop {d}")),
        }
    }
}
