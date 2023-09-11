use std::{collections::HashMap, rc::Rc};

use crate::shared::types::QuectoType;

use super::Destination64;

#[derive(Clone, Debug)]
pub struct Scope
{
    current_offset: usize,
    variable_stack: Vec<(String, usize, QuectoType)>,
    parent_scope : Option<Box<Scope>>
}

impl Scope
{
    pub fn new() -> Self
    {
        Scope {
            current_offset: 0,
            variable_stack: Vec::new(),
            parent_scope: None
        }
    }

    pub fn set_parent(&mut self, parent : &mut Scope)
    {
        self.parent_scope = Some(Box::new(parent.clone()));
    }

    pub fn add_variable(&mut self, variable: String, size_in_bytes: usize, q_type : QuectoType) -> Option<usize>
    {
        self.current_offset += size_in_bytes;

        self.variable_stack.insert(0, (variable, size_in_bytes, q_type));

        Some(self.current_offset)
    }

    pub fn get_variable<S : AsRef<str>>(&self, variable: S) -> Option<(usize, QuectoType)>
    {
        let variable = variable.as_ref().to_string();
        for var in &self.variable_stack
        {
            if var.0 == variable
            {
                return Some((var.1, var.2));
            }
        }
        
        if let Some(scope) = &self.parent_scope
        {
            scope.get_variable(variable)
        } else
        {
            None
        }
    }
}
