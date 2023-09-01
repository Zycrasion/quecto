

pub enum QuectoToken
{
    Return(Box<QuectoToken>),
    FunctionDeclaration(QuectoType, String),
    Operand(QuectoOperand, Box<QuectoToken>, Box<QuectoToken>),
}