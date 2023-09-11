use std::fmt::Display;

use crate::shared::types::{QuectoOperand, QuectoType};

#[derive(Debug, PartialEq)]

pub enum ModuleType
{
    Main,
    Library,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum QuectoValue
{
    QWORD(u64),
    DWORD(u32),
    WORD(u16),
    BYTE(u8),
}

impl QuectoValue
{
    pub fn get_value(&self) -> u64
    {
        match self
        {
            QuectoValue::BYTE(i) => *i as u64,
            QuectoValue::WORD(i) => *i as u64,
            QuectoValue::DWORD(i) => *i as u64,
            QuectoValue::QWORD(i) => *i,
        }
    }

    pub fn get_size_in_bytes(&self) -> usize
    {
        match self
        {
            QuectoValue::QWORD(_) => 8,
            QuectoValue::DWORD(_) => 4,
            QuectoValue::WORD(_) => 2,
            QuectoValue::BYTE(_) => 1,
        }
    }
}

impl Display for QuectoValue
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.get_value())
    }
}

#[derive(Debug, PartialEq)]
pub enum QuectoNode
{
    Scope(Vec<QuectoNode>, usize),
    Return(Box<QuectoNode>),
    FunctionDeclaration(QuectoType, String, Vec<QuectoNode>, usize),
    FunctionCall(String),
    Operand(QuectoOperand, Box<QuectoNode>, Box<QuectoNode>),
    Value(QuectoValue, QuectoType),
    VariableDeclaration(String, QuectoValue, QuectoType, usize),
    IdentifierReference(String),
    Module(ModuleType, Vec<QuectoNode>),
}
