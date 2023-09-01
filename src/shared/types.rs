use std::str::FromStr;

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
            _ => Err(()),
        }
    }
}
