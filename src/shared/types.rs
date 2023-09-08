use std::{fmt::Display, str::FromStr};

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
    Qstr,
}

impl Display for QuectoType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "{}",
            match self
            {
                QuectoType::Qu8 => "u8",
                QuectoType::Qu16 => "u16",
                QuectoType::Qu32 => "u32",
                QuectoType::Qu64 => "u64",
                QuectoType::Qi8 => "i8",
                QuectoType::Qi16 => "i16",
                QuectoType::Qi32 => "i32",
                QuectoType::Qi64 => "i64",
                QuectoType::Qf32 => "f32",
                QuectoType::Qf64 => "f64",
                QuectoType::Qbool => "bool",
                QuectoType::Qchar => "char",
                QuectoType::Qstr => "str",
            }
        )
    }
}

impl QuectoType
{
    pub fn is_int(&self) -> bool
    {
        match self
        {
            Self::Qi64 | Self::Qi32 | Self::Qi16 | Self::Qi8 => true,
            _ => false,
        }
    }

    pub fn is_unsigned_int(&self) -> bool
    {
        match self
        {
            Self::Qu64 | Self::Qu32 | Self::Qu16 | Self::Qu8 => true,
            _ => false,
        }
    }

    pub fn is_float(&self) -> bool
    {
        match self
        {
            Self::Qf64 | Self::Qf32 => true,
            _ => false,
        }
    }
}

impl FromStr for QuectoType
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "u8" => Ok(QuectoType::Qu8),
            "u16" => Ok(QuectoType::Qu16),
            "u32" => Ok(QuectoType::Qu32),
            "u64" => Ok(QuectoType::Qu64),

            "i8" => Ok(QuectoType::Qi8),
            "i16" => Ok(QuectoType::Qi16),
            "i32" => Ok(QuectoType::Qi32),
            "i64" => Ok(QuectoType::Qi64),

            "f32" => Ok(QuectoType::Qf32),
            "f64" => Ok(QuectoType::Qf64),

            "bool" => Ok(QuectoType::Qbool),
            "char" => Ok(QuectoType::Qchar),
            "str" => Ok(QuectoType::Qstr),

            _ => Err(()),
        }
    }
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
    Not,
}

impl FromStr for QuectoOperand
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            "%" => Ok(Self::Modulus),

            "||" => Ok(Self::Or),
            "&&" => Ok(Self::And),
            "!" => Ok(Self::Not),
            _ => Err(()),
        }
    }
}

impl ToString for QuectoOperand
{
    fn to_string(&self) -> String
    {
        match self
        {
            QuectoOperand::Add => "+",
            QuectoOperand::Subtract => "-",
            QuectoOperand::Multiply => "*",
            QuectoOperand::Divide => "/",
            QuectoOperand::Modulus => "%",
            QuectoOperand::Or => "||",
            QuectoOperand::And => "&&",
            QuectoOperand::Not => "!",
        }
        .to_string()
    }
}
