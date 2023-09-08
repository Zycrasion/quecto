use std::fmt::Display;

use super::{SupportedSystems, Asmx86_64, Destination64, Source64, SystemCalls};

pub struct Programx86_64
{
    pub system : SupportedSystems,
    asm : Vec<Asmx86_64>
}

impl Programx86_64
{
    pub fn new(system : SupportedSystems) -> Self
    {
        Programx86_64 { system, asm: Vec::new() }
    }

    pub fn mov(&mut self, dest : Destination64, source : Source64)
    {
        self.asm.push(Asmx86_64::Mov(dest, source));
    }

    pub fn syscall(&mut self, call : SystemCalls)
    {
        call.to_asm(self)
    }

    pub fn _syscall(&mut self)
    {
        self.asm.push(Asmx86_64::Syscall);
    }

    pub fn label<S : AsRef<str>>(&mut self, name : S)
    {
        self.asm.push(Asmx86_64::Label(name.as_ref().to_owned()))
    }

    pub fn global_label<S : AsRef<str>>(&mut self, name : S)
    {
        self.asm.push(Asmx86_64::Global(name.as_ref().to_owned()))
    }

    pub fn section<S : AsRef<str>>(&mut self, name : S)
    {
        self.asm.push(Asmx86_64::Section(name.as_ref().to_owned()))
    }

    pub fn call<S : AsRef<str>>(&mut self, name : S)
    {
        self.asm.push(Asmx86_64::Call(name.as_ref().to_owned()))
    }

    pub fn ret(&mut self)
    {
        self.asm.push(Asmx86_64::Return)
    }

    pub fn push(&mut self, source : Source64)
    {
        self.asm.push(Asmx86_64::Push(source))
    }

    pub fn pop(&mut self, dest : Destination64)
    {
        self.asm.push(Asmx86_64::Pop(dest))
    }
}

impl Display for Programx86_64
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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