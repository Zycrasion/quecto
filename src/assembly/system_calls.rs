use super::{program::Programx86_64, AsmValue, Register64};

#[derive(PartialEq, Clone, Debug)]
pub enum SystemCalls
{
    Exit(AsmValue),
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
                program.mov(AsmValue::Reg(Register64::Rdi), exit_code.clone());
                program.mov(AsmValue::Reg(Register64::Rax), AsmValue::Imm(60));
                program._syscall();
            }
        }
    }
}
