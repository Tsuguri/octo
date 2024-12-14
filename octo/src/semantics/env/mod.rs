use parser::ast::*;
use std::cell::RefCell;
use std::clone::Clone as _;

#[derive(Debug)]
pub struct Function {
    name: String,
    arguments: Vec<Variable>,
    results: Vec<Type>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub span: Span,
    pub typ: Type,
    pub used: bool,
}

#[derive(Debug)]
pub struct Scope<'a> {
    pub variables: RefCell<Vec<Variable>>,
    parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    pub fn global<'b>() -> Scope<'b> {
        Scope {
            variables: RefCell::new(vec![]),
            parent: None,
        }
    }

    pub fn child_scope(&self) -> Scope {
        Scope {
            variables: RefCell::new(vec![]),
            parent: Some(self),
        }
    }
    pub fn variable_exists(&self, name: &str) -> Option<Span> {
        match self.variables.borrow().iter().find(|x| x.name == name) {
            None => match self.parent {
                None => None,
                Some(parent) => parent.variable_exists(name),
            },
            Some(x) => Some(x.span),
        }
    }

    pub fn create_variable(&mut self, name: &str, typ: Type, span: Span) -> Result<(), Span> {
        match self.variable_exists(name) {
            Some(span) => return Result::Err(span),
            None => {}
        };
        self.variables.borrow_mut().push(Variable {
            name: name.to_owned(),
            typ,
            span,
            used: false,
        });
        Result::Ok(())
    }

    pub fn use_variable(&self, name: &str) -> Option<Type> {
        let mut borr = self.variables.borrow_mut();
        let variable = borr.iter_mut().find(|x| x.name == name);
        match variable {
            None => match self.parent {
                None => None,
                Some(parent) => parent.use_variable(name),
            },
            Some(x) => {
                x.used = true;
                Some(x.typ.clone())
            }
        }
    }

    pub fn unused_variables(&self) -> Vec<Variable> {
        let v = self.variables.borrow();
        let vars = v.iter().filter(|x| x.used == false).map(|x| x.clone());
        match self.parent {
            None => vars.collect(),
            Some(parent) => {
                let mut v = parent.unused_variables();
                v.extend(vars);
                v
            }
        }
    }

    //    pub fn add_function(&mut self, func: &GpuFunction) {
    //        self.functions.push(Function{
    //            name: func.name.val.to_owned(),
    //            arguments: func.arguments.clone(),
    //            results: func.results.iter().map(|x| x.typ.clone()).collect(),
    //        });
    //    }
}
