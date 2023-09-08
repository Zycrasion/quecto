use std::{iter::Peekable, vec::IntoIter, process::id};

use crate::parser::{ModuleType, Parser, QuectoNode};

use super::{Asmx86_64, Destination64, Register64, Source64, SupportedSystems, SystemCalls, program::Programx86_64};

#[derive(Clone)]
pub struct Compiler(pub Parser);

impl Compiler
{
    fn compile_node(
        nodes: &mut Peekable<IntoIter<QuectoNode>>,
        program: &mut Programx86_64
    )
    {
        match nodes.next().unwrap()
        {
            QuectoNode::IdentifierReference(_) => todo!(),
            QuectoNode::Scope(_) => todo!(),
            QuectoNode::Return(val) =>
            {
                if let QuectoNode::Value(v) = *val
                {
                    program.pop(Destination64::Reg(Register64::Rbp));
                    program.mov(Destination64::Reg(Register64::Rax), Source64::Imm(v.get_value()));
                    program.ret();
                } else
                {
                    todo!()
                }
            }
            QuectoNode::FunctionDeclaration(_return_type, name, executable) =>
            {
                program.label(name);
                program.push(Source64::Reg(Register64::Rbp));
                program.mov(
                    Destination64::Reg(Register64::Rbp), 
                    Source64::Reg(Register64::Rsp)
                );

                if let QuectoNode::Scope(nodes) = *executable
                {
                    let mut nodes = nodes.into_iter().peekable();
                    while let Some(_) = nodes.peek()
                    {
                        Compiler::compile_node(&mut nodes, program);
                    }
                }
                else
                {
                    panic!("EXPECTED SCOPE AFTER FUNCTION");
                }
            }
            QuectoNode::Operand(_, _, _) => todo!(),
            QuectoNode::Value(_) => todo!(),
            QuectoNode::VariableDeclaration(_, _) => todo!(),
            QuectoNode::Module(_, _) => todo!(),
            QuectoNode::FunctionCall(identifier) => program.call(identifier),
        }
    }

    pub fn compile(self) -> String
    {
        let _module_type = ModuleType::Main;
        let mut program = Programx86_64::new(SupportedSystems::Linux);

        let nodes = self.0.parse();

        if let QuectoNode::Module(_mod_type, nodes) = nodes
        {
            program.section(".text");
            program.global_label("_start");
            program.label("_start");
            program.call("start");
            program.syscall(SystemCalls::Exit(Source64::Reg(Register64::Rax)));

            let mut nodes = nodes.into_iter().peekable();

            while let Some(_) = nodes.peek()
            {
                Compiler::compile_node(&mut nodes, &mut program);
            }
        }
        else
        {
            eprintln!("Literally How?");
            panic!()
        }

        return program.to_string();
    }
}
