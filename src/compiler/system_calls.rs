use super::{program::{Programx86_64}, Destination64, Register64, Source64};

#[derive(PartialEq, Clone, Debug)]
pub enum SystemCalls
{
    Exit(Source64),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SupportedSystems
{
    Linux,
}

impl SystemCalls
{
    pub fn to_asm(&self, program: &mut Programx86_64)
    {
        match program.system
        {
            SupportedSystems::Linux => self._linux_asm(program),
        }
    }

    fn _linux_asm(&self, program: &mut Programx86_64)
    {
        match self
        {
            SystemCalls::Exit(exit_code) => 
            {
                program.mov(
                    Destination64::Reg(Register64::Rdi),
                    exit_code.clone()
                );
                program.mov(
                    Destination64::Reg(Register64::Rax),
                    Source64::Imm(60)
                );
                program._syscall();
            },
        }
    }
}
