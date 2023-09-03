use crate::{parser::{Parser, QuectoNode, ModuleType}, shared::types::QuectoNumberTypes};

use super::{Assembly, Destination, Register, Source};

#[derive(Clone)]
pub struct Compiler(pub Parser);

impl Compiler
{
    pub fn compile(self) -> String
    {
        let mut module_type = ModuleType::Main;
        let mut assembly = Vec::new();

        let nodes = self.0.parse();

        if let QuectoNode::Module(mod_type, nodes) = nodes
        {
            assembly.push(Assembly::Section(".text".to_string()));
            assembly.push(Assembly::Global("_start".to_string()));
            assembly.push(Assembly::Label("_start".to_string()));

            for node in nodes.iter()
            {
                match node
                {
                    QuectoNode::Scope(_) => todo!(),
                    QuectoNode::Return(val) => 
                    {
                        if let QuectoNode::IntLiteral(i) = **val
                        {
                            assembly.push(Assembly::Mov(Destination::Reg(Register::Rax), Source::Imm(QuectoNumberTypes::Qi64(60))));
                            assembly.push(Assembly::Mov(Destination::Reg(Register::Rdi), Source::Imm(QuectoNumberTypes::Qi64(i))));
                            assembly.push(Assembly::Syscall);
                        }
                    },
                    QuectoNode::FunctionDeclaration(..) => todo!(),
                    QuectoNode::Operand(_, _, _) => todo!(),
                    QuectoNode::FloatLiteral(_) => todo!(),
                    QuectoNode::IntLiteral(_) => todo!(),
                    QuectoNode::Value(_) => todo!(),
                    QuectoNode::VariableDeclaration(_, _) => todo!(),
                    QuectoNode::Module(_, _) => todo!(),
                }
            }
            
        } else {
            eprintln!("Literally How?");
            panic!()
        }

        let mut program = String::new();

        for asm in assembly
        {
            program += &asm.to_string();
            program.push('\n');
        }

        return program;
    }
}