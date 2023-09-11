use std::{iter::Peekable, vec::IntoIter};

use crate::parser::{ModuleType, Parser, QuectoNode};

use super::{
    program::Programx86_64, Destination64, Register64, Source64, SupportedSystems, SystemCalls, scope::Scope,
};

#[derive(Clone)]
pub struct Compiler(pub Parser);

impl Compiler
{
    fn compile_node(nodes: &mut Peekable<IntoIter<QuectoNode>>, program: &mut Programx86_64, scope : &mut Scope)
    {
        match nodes.next().unwrap()
        {
            QuectoNode::IdentifierReference(_) => todo!(),
            QuectoNode::Scope(nodes, reseserve_space) => 
            {
                todo!();
                let mut new_scope = Scope::new();
                new_scope.set_parent(scope);

                let mut nodes = nodes.into_iter().peekable();
                while let Some(_) = nodes.peek()
                {
                    Compiler::compile_node(&mut nodes, program, scope);
                }
            },
            QuectoNode::Return(val) =>
            {
                if let QuectoNode::Value(v, qtype) = *val
                {
                    program.pop(Destination64::Reg(Register64::Rbp));
                    program.mov(
                        Destination64::Reg(Register64::Rax),
                        Source64::Imm(v.get_value()),
                    );
                    program.ret();
                }
                else
                {
                    todo!()
                }
            }
            QuectoNode::FunctionDeclaration(_return_type, name, nodes, reserve_space) =>
            {
                program.label(name);
                program.push(Source64::Reg(Register64::Rbp));
                program.mov(
                    Destination64::Reg(Register64::Rbp),
                    Source64::Reg(Register64::Rsp),
                );

                let mut new_scope = Scope::new();
                new_scope.set_parent(scope);

                let mut nodes = nodes.into_iter().peekable();
                while let Some(_) = nodes.peek()
                {
                    Compiler::compile_node(&mut nodes, program, scope);
                }
            }
            QuectoNode::Operand(_, _, _) => todo!(),
            QuectoNode::Value(_, _) => todo!(),
            QuectoNode::VariableDeclaration(name, v, t, offset) =>
            {
                if let Some(offset) = scope.add_variable(name, v.get_size_in_bytes(), t)
                {
                    // Stack grows downwards
                    program.mov(Destination64::MemRegOffset(Register64::Rbp, 0 - offset), Source64::Imm(v.get_value()));
                }
            },
            QuectoNode::Module(_, _) => todo!(),
            QuectoNode::FunctionCall(identifier) => program.call(identifier),
        }
    }

    pub fn compile(self) -> String
    {
        let _module_type = ModuleType::Main;
        let mut program = Programx86_64::new(SupportedSystems::Linux);
        let mut scope = Scope::new();
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
                Compiler::compile_node(&mut nodes, &mut program, &mut scope);
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
