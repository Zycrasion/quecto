use crate::shared::types::{QuectoType, QuectoOperand, QuectoTypeContainer, QuectoNumberTypes};

pub enum QuectoNode
{
    Scope(Vec<QuectoNode>),
    Return(Box<QuectoNode>),
    FunctionDeclaration(QuectoType, String),
    Operand(QuectoOperand, Box<QuectoNode>, Box<QuectoNode>),
    Number(QuectoNumberTypes),
    VariableDeclaration(String, QuectoTypeContainer)
}