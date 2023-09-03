use crate::shared::types::QuectoNumberTypes;

use super::{Assembly, Destination, Register, Source};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SystemCalls
{
    Exit,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SupportedSystems
{
    Linux,
}

impl SystemCalls
{
    pub fn to_number(&self, system: SupportedSystems) -> i64
    {
        match self
        {
            SystemCalls::Exit =>
            {
                if system == SupportedSystems::Linux
                {
                    60
                }
                else
                {
                    panic!("None, Yet");
                }
            }
        }
    }

    pub fn to_asm(&self, system: SupportedSystems) -> Assembly
    {
        match system
        {
            SupportedSystems::Linux =>
            {
                let number = self.to_number(system);
                Assembly::Mov(
                    Destination::Reg(Register::Rax),
                    Source::Imm(QuectoNumberTypes::Qi64(number)),
                )
            }
        }
    }
}
