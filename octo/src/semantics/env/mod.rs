use parser::ast::*;
use std::clone::Clone as _;

#[derive(Debug)]
pub struct Function {
    name: String,
    arguments: Vec<Variable>,
    results: Vec<Type>,
}

#[derive(Debug)]
pub struct Scope<'a> {
    pub functions: Vec<Function>,
    pub variables: Vec<(String, Type)>,
    parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn global<'b>() -> Scope<'b> {
        Scope {
            functions: vec![],
            variables: vec![],
            parent: None,
        }
    }

    pub fn child_scope(&self) -> Scope {
        Scope {
            functions: vec![],
            variables: vec![],
            parent: Some(self),
        }
    }

    pub fn add_function(&mut self, func: &GpuFunction) {
        self.functions.push(Function{
            name: func.name.val.to_owned(),
            arguments: func.arguments.clone(),
            results: func.results.iter().map(|x| x.typ.clone()).collect(),
        });
    }
}
