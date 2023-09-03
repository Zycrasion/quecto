use std::{vec::IntoIter, iter::Peekable, f32::consts::E};

use crate::{parser::{Parser, QuectoNode, ModuleType}, shared::types::QuectoNumberTypes};

use super::{Assembly, Destination, Register, Source, SupportedSystems, SystemCalls};

#[derive(Clone)]
pub struct Compiler(pub Parser);

impl Compiler
{
    fn compile_node(nodes: &mut Peekable<IntoIter<QuectoNode>>, system : SupportedSystems) -> Vec<Assembly>
    {
        let mut assembly = vec![];
        match nodes.next().unwrap()
        {
            QuectoNode::Scope(_) => todo!(),
            QuectoNode::Return(val) => 
            {
                if let QuectoNode::IntLiteral(i) = *val
                {
                    assembly.push(Assembly::Mov(Destination::Reg(Register::Rax), Source::Imm(QuectoNumberTypes::Qi64(i))));
                    assembly.push(Assembly::Return);
                }
            },
            QuectoNode::FunctionDeclaration(return_type, name, executable) => 
            {
                assembly.push(Assembly::Label(name.to_string()));
                if let QuectoNode::Scope(nodes) = *executable
                {
                    let mut nodes = nodes.into_iter().peekable();
                    while let Some(_) = nodes.peek()
                    {
                        assembly.append(&mut Compiler::compile_node(&mut nodes, system));
                        nodes.next();
                    }
                } else {panic!("EXPECTED SCOPE AFTER FUNCTION");}
            },
            QuectoNode::Operand(_, _, _) => todo!(),
            QuectoNode::FloatLiteral(_) => todo!(),
            QuectoNode::IntLiteral(_) => todo!(),
            QuectoNode::Value(_) => todo!(),
            QuectoNode::VariableDeclaration(_, _) => todo!(),
            QuectoNode::Module(_, _) => todo!(),
        }
        assembly
    }

    pub fn compile(self) -> String
    {
        let mut module_type = ModuleType::Main;
        let mut assembly = Vec::new();
        let system = SupportedSystems::Linux;

        let nodes = self.0.parse();

        if let QuectoNode::Module(mod_type, nodes) = nodes
        {
            assembly.push(Assembly::Section(".text".to_string()));
            assembly.push(Assembly::Global("_start".to_string()));
            assembly.push(Assembly::Label("_start".to_string()));
            assembly.push(Assembly::Call("start".to_owned()));
            

            assembly.push(Assembly::Mov(Destination::Reg(Register::Rdi), Source::Reg(Register::Rax)));
            assembly.push(SystemCalls::Exit.to_asm(system));
            assembly.push(Assembly::Syscall);

            let mut nodes = nodes.into_iter().peekable();

            while let Some(_) = nodes.peek()
            {
                assembly.append(&mut Compiler::compile_node(&mut nodes, system));
                nodes.next();
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