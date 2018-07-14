use octo_parser::ast::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Scope {
    pub types: Vec<Type>,
    pub functions: Vec<(String, Option<Type>, Vec<Type>)>,
}

#[derive(Debug)]
pub struct Env {
    pub static_scope: Scope,
    pub local_scopes: Vec<Scope>,
}

pub struct ScopeToken {
    _phantom: PhantomData<i32>,
}

impl ScopeToken {
    fn new() -> ScopeToken {
        ScopeToken {
            _phantom: PhantomData,
        }
    }
}

impl Env {
    pub fn new() -> Env {
        Env {
            static_scope: Scope {
                types: vec![Type::Float, Type::Int, Type::Bool, Type::String],
                functions: vec![
                    ("print".to_owned(), None, vec![Type::String]),
                    ("sin".to_owned(), Some(Type::Float), vec![Type::Float]),
                    ("cos".to_owned(), Some(Type::Float), vec![Type::Float]),
                ],
            },
            local_scopes: vec![],
        }
    }

    pub fn check_type(&self, typ: &Type) -> bool {
        self.static_scope.types.contains(typ)
    }

    pub fn push_scope(&mut self) -> ScopeToken {
        self.local_scopes.push(Scope {
            types: vec![],
            functions: vec![],
        });
        ScopeToken::new()
    }

    pub fn pop_scope(&mut self, _token: ScopeToken) {
        self.local_scopes.pop();
    }

    pub fn add_function(&mut self, func: &Function, token: &ScopeToken) {
        let scope = self.local_scopes.pop();
        match scope {
            None => unreachable!(),
            Some(mut scp) => {
                //
                scp.functions.push((
                    func.name.to_owned(),
                    func.ret.clone(),
                    func.arguments.iter().map(|x| x.1.clone()).collect(),
                ));
                self.local_scopes.push(scp);
            }
        }
    }
}
