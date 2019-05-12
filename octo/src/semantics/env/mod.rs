use parser::ast::*;

#[derive(Debug)]
pub struct Scope<'a> {
    pub types: Vec<Type>,
    pub functions: Vec<(String, Vec<Type>)>,
    pub variables: Vec<(String, Type)>,
    parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn global<'b>() -> Scope<'b> {
        Scope {
            types: vec![Type::Float, Type::Int, Type::Bool, Type::String],
            functions: vec![],
            variables: vec![],
            parent: None,
        }
    }

    pub fn child_scope<'b>(parent: &'b Scope) -> Scope<'b> {
        Scope {
            types: vec![],
            functions: vec![],
            variables: vec![],
            parent: Some(parent),
        }
    }

    pub fn add_function(&mut self, func: &GpuFunction) {
        self.functions.push((
            func.name.val.to_owned(),
            func.arguments.iter().map(|x| x.typ.clone()).collect(),
        ));
    }
}
