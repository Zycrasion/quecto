use crate::shared::types::{QuectoNumberTypes, QuectoOperand, QuectoType, QuectoTypeContainer};

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
    Operand(QuectoOperand, Box<QuectoNode>, Box<QuectoNode>),
    FloatLiteral(f64),
    IntLiteral(i64),
    Value(QuectoTypeContainer),
    VariableDeclaration(String, QuectoTypeContainer),
    Module(ModuleType, Vec<QuectoNode>),
}
