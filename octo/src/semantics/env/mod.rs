use parser::ast::*;

#[derive(Debug)]
pub struct Scope<'a> {
    pub types: Vec<Type>,
    pub functions: Vec<(String, Option<Type>, Vec<Type>)>,
    pub variables: Vec<(String, Type)>,
    parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn global<'b>() -> Scope<'b> {
        Scope {
            types: vec![Type::Float, Type::Int, Type::Bool, Type::String],
            functions: vec![
                ("print".to_owned(), None, vec![Type::String]),
                ("sin".to_owned(), Some(Type::Float), vec![Type::Float]),
                ("cos".to_owned(), Some(Type::Float), vec![Type::Float]),
            ],
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

    pub fn check_type(&self, typ: &Type) -> bool {
        match self.types.contains(typ) {
            true => true,
            false => match self.parent {
                None => false,
                Some(parent) => parent.check_type(typ),
            },
        }
    }

    pub fn add_function(&mut self, func: &Function) {
        self.functions.push((
            func.name.to_owned(),
            func.ret.clone(),
            func.arguments.iter().map(|x| x.1.clone()).collect(),
        ));
    }

    pub fn add_variable(&mut self, name: &str, typ: Type) {
        self.variables.push((name.to_owned(), typ));
    }

    pub fn check_variable(&self, name: &str) -> Option<Type> {
        match self.variables.iter().find(|x| x.0 == name) {
            None => match self.parent {
                None => None,
                Some(parent) => parent.check_variable(name),
            },
            Some(x) => Some(x.1.clone()),
        }
    }
}
