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
}

impl Display for QuectoValue
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "{}",
            self.get_value()
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum QuectoNode
{
    Scope(Vec<QuectoNode>),
    Return(Box<QuectoNode>),
    FunctionDeclaration(QuectoType, String, Box<QuectoNode>),
    FunctionCall(String),
    Operand(QuectoOperand, Box<QuectoNode>, Box<QuectoNode>),
    Value(QuectoValue),
    VariableDeclaration(String, QuectoValue),
    IdentifierReference(String),
    Module(ModuleType, Vec<QuectoNode>),
}
