use crate::shared::types::{QuectoType, QuectoOperand};

#[derive(Clone, PartialEq, Debug)]
pub enum QuectoToken
{
    IntLiteral(i64),
    FloatLiteral(f64), // Why is it 64 bits?, because all types will fit into 4 bytes therefore to reduce code complexity every value goes to the highest precision one available then casted down
    BoolLiteral(bool),
    CharLiteral(char),
    StringLiteral(String),
    Identifier(String),
    Colon,
    SemiColon,
    OtherPunctuation(char),
    Type(QuectoType),
    Operand(QuectoOperand),
    Unknown
}

impl QuectoToken
{
    pub fn unwrap(&self)
    {
        if self.is_unknown()
        {
            panic!();
        }
    }

    pub fn is_unknown(&self) -> bool
    {
        return self == &QuectoToken::Unknown
    }
}