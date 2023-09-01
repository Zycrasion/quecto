#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum QuectoType
{
    Qu8,
    Qu16,
    Qu32,
    Qu64,
    Qi8,
    Qi16,
    Qi32,
    Qi64,
    Qf32,
    Qf64,
    Qbool,
    Qchar,
    Qstr
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum QuectoOperand
{
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    Or,
    And,
    Not
}