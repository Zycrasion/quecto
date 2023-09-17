use std::fmt::Display;

use super::{Asmx86_64, AsmValue, SupportedSystems, SystemCalls};

macro_rules! instr {
    ($name:ident, $instr_name:ident, two) => {
        pub fn $name(&mut self, dest: AsmValue, source: AsmValue)
        {
            self.asm.push(Asmx86_64::$instr_name(dest, source));
        }
    };
    ($name:ident, $instr_name:ident, one) => {
        pub fn $name(&mut self, a: AsmValue)
        {
            self.asm.push(Asmx86_64::$instr_name(a));
        }
    };
    ($name:ident, $instr_name:ident, s) => {
        pub fn $name<S: AsRef<str>>(&mut self, a: S)
        {
            self.asm.push(Asmx86_64::$instr_name(a.as_ref().to_string()));
        }
    };
    ($name:ident, $instr_name:ident, $type:ident) => {
        pub fn $name(&mut self, a: $type)
        {
            self.asm.push(Asmx86_64::$instr_name(a));
        }
    };
    ($name:ident, $instr_name:ident) => {
        pub fn $name(&mut self)
        {
            self.asm.push(Asmx86_64::$instr_name);
        }
    };
}

pub struct Programx86_64
{
    pub system: SupportedSystems,
    asm: Vec<Asmx86_64>,
}

impl Programx86_64
{
    pub fn new(system: SupportedSystems) -> Self
    {
        Programx86_64 {
            system,
            asm: Vec::new(),
        }
    }

    pub fn syscall(&mut self, call: SystemCalls)
    {
        call.to_asm(self)
    }
    
    instr!(_syscall, Syscall);
    instr!(ret, Return);

    instr!(label, Label, s);
    instr!(global_label, Global, s);
    instr!(section, Section, s);
    instr!(call, Call, s);

    instr!(push, Push, one);
    instr!(pop, Pop, one);

    instr!(mov, Mov, two);
    instr!(sub, Sub, two);
    instr!(add, Add, two);
}

impl Display for Programx86_64
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", {
            let mut program = String::new();
            for asm in &self.asm
            {
                program += &asm.to_string();
                program.push('\n');
            }
            program
        })
    }
}
