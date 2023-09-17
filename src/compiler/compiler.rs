use std::{iter::Peekable, vec::IntoIter};

use crate::parser::{ModuleType, Parser, QuectoNode};

use crate::assembly::{
    Programx86_64, AsmValue, Register64, SupportedSystems, SystemCalls,
};

#[derive(Clone)]
pub struct Compiler(pub Parser);

impl Compiler
{

    fn compile_as_value(node: &QuectoNode, program: &mut Programx86_64) -> AsmValue
    {
        match node
        {
            QuectoNode::Scope(_, _) => todo!(),
            QuectoNode::Return(_) => todo!(),
            QuectoNode::FunctionDeclaration(_, _, _, _) => todo!(),
            QuectoNode::FunctionCall(s) => 
            {
                program.call(s);
                AsmValue::Reg(Register64::Rax)
            },
            QuectoNode::Operand(_, _, _) => todo!(),
            QuectoNode::Value(v, _t) => 
            {
                AsmValue::Imm(v.get_value())
            },
            QuectoNode::VariableDeclaration(_, _, _, _) => panic!(),
            QuectoNode::IdentifierReference(_) => todo!(),
            QuectoNode::Module(_, _) => todo!(),
        }
    }

    fn compile_node(nodes: &mut Peekable<IntoIter<QuectoNode>>, program: &mut Programx86_64)
    {
        match nodes.next().unwrap()
        {
            QuectoNode::IdentifierReference(_) => todo!(),
            QuectoNode::Scope(nodes, reserve_space) => 
            {
                program.push(AsmValue::Reg(Register64::Rbp));
                program.mov(
                    AsmValue::Reg(Register64::Rbp),
                    AsmValue::Reg(Register64::Rsp),
                );
                program.sub(AsmValue::Reg(Register64::Rsp), AsmValue::Imm(reserve_space as u64));

                let mut nodes = nodes.into_iter().peekable();
                while let Some(_) = nodes.peek()
                {
                    Compiler::compile_node(&mut nodes, program);
                }

                program.add(AsmValue::Reg(Register64::Rsp), AsmValue::Imm(reserve_space as u64));
                program.pop(AsmValue::Reg(Register64::Rbp));
                program.ret();
            },
            QuectoNode::Return(val) =>
            {
                let value = Self::compile_as_value(&val, program);
                program.mov(AsmValue::Reg(Register64::Rax), value);
            }
            QuectoNode::FunctionDeclaration(_return_type, name, nodes, reserve_space) =>
            {
                program.label(name);
                program.push(AsmValue::Reg(Register64::Rbp));
                program.mov(
                    AsmValue::Reg(Register64::Rbp),
                    AsmValue::Reg(Register64::Rsp),
                );
                program.sub(AsmValue::Reg(Register64::Rsp), AsmValue::Imm(reserve_space as u64));

                let mut nodes = nodes.into_iter().peekable();
                while let Some(_) = nodes.peek()
                {
                    Compiler::compile_node(&mut nodes, program);
                }
                program.add(AsmValue::Reg(Register64::Rsp), AsmValue::Imm(reserve_space as u64));
                program.pop(AsmValue::Reg(Register64::Rbp));
                program.ret();
            }
            QuectoNode::Operand(_, _, _) => todo!(),
            QuectoNode::Value(_, _) => todo!(),
            QuectoNode::VariableDeclaration(name, v, t, offset) =>
            {
                program.mov(AsmValue::MemRegOffset(Register64::Rbp, 0 - offset as isize), AsmValue::Imm(v.get_value()));
            },
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
            program.syscall(SystemCalls::Exit(AsmValue::Reg(Register64::Rax)));

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
