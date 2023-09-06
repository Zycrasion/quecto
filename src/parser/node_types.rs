use crate::shared::types::{QuectoOperand, QuectoType, QuectoTypeContainer};

#[derive(Debug, PartialEq)]

pub enum ModuleType
{
    Main,
    Library,
}

#[derive(Debug, PartialEq)]
pub enum QuectoNode
{
    Scope(Vec<QuectoNode>),
    Return(Box<QuectoNode>),
    FunctionDeclaration(QuectoType, String, Box<QuectoNode>),
    FunctionCall(String),
    Operand(QuectoOperand, Box<QuectoNode>, Box<QuectoNode>),
    Value(QuectoTypeContainer),
    VariableDeclaration(String, QuectoTypeContainer),
    IdentifierReference(String),
    Module(ModuleType, Vec<QuectoNode>),
}
