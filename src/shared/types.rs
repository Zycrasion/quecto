use std::{str::FromStr, fmt::Display};

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            QuectoType::Qu8     => "u8",
            QuectoType::Qu16    => "u16",
            QuectoType::Qu32    => "u32",
            QuectoType::Qu64    => "u64",
            QuectoType::Qi8     => "i8",
            QuectoType::Qi16    => "i16",
            QuectoType::Qi32    => "i32",
            QuectoType::Qi64    => "i64",
            QuectoType::Qf32    => "f32",
            QuectoType::Qf64    => "f64",
            QuectoType::Qbool   => "bool",
            QuectoType::Qchar   => "char",
            QuectoType::Qstr    => "str",
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum QuectoTypeContainer
{
    Qu8(u8),
    Qu16(u16),
    Qu32(u32),
    Qu64(u64),
    Qi8(i8),
    Qi16(i16),
    Qi32(i32),
    Qi64(i64),
    Qf32(f32),
    Qf64(f64),
    Qbool(bool),
    Qchar(char),
    Qstr(String),
}

#[derive(Clone, PartialEq, Debug)]
pub enum QuectoNumberTypes
{
    Qu8(u8),
    Qu16(u16),
    Qu32(u32),
    Qu64(u64),
    Qi8(i8),
    Qi16(i16),
    Qi32(i32),
    Qi64(i64),
    Qf32(f32),
    Qf64(f64),
}

impl Display for QuectoNumberTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            QuectoNumberTypes::Qu8(v)   => v.to_string(),
            QuectoNumberTypes::Qu16(v)  => v.to_string(),
            QuectoNumberTypes::Qu32(v)  => v.to_string(),
            QuectoNumberTypes::Qu64(v)  => v.to_string(),
            QuectoNumberTypes::Qi8(v)   => v.to_string(),
            QuectoNumberTypes::Qi16(v)  => v.to_string(),
            QuectoNumberTypes::Qi32(v)  => v.to_string(),
            QuectoNumberTypes::Qi64(v)  => v.to_string(),
            QuectoNumberTypes::Qf32(v)  => v.to_string(),
            QuectoNumberTypes::Qf64(v)  => v.to_string(),
        };

        write!(f, "{val}")
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
