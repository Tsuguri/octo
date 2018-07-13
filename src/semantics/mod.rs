use octo_parser::ast;


pub enum SemanticError {
    UndefinedIdentifier,
    UnusedArgument,
    TypeMismatch,
    ArgumentsNumberMismatch,


}
#[derive(Debug)]
pub enum Stack<p> {
    None,
    Some(p,Box<Stack<p>>)
}

impl<p> Stack<p> {
    pub fn push(self, value: p)->Stack<p>{
        Stack::Some(value, Box::new(self))
    }

    pub fn pop(self) -> (Stack<p>,Option<p>) {
        match self {
            Stack::None => (Stack::None, None),
            Stack::Some(val, child) => (*child, Some(val)),
        
        }
    }

}

pub struct Scope {
    //
}

impl Scope {
    pub fn heh(self) {
        let mut stk = Stack::None;
        stk = stk.push(1);
        stk = stk.push(3.0);
        stk = stk.push(2);
        println!("{:?}",stk);
        let (stk,value) = stk.pop();
        println!("{:?}",value);
        println!("{:?}",stk);
    }


}

pub struct Env {
    scopes: Stack<Scope>,
}
