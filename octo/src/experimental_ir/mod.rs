
#[derive(Debug)]
pub struct Param{
    pub id: usize,

}

pub enum BinOperator {
    Add,
    Mul,
    Div,
    Mod,
    Sub,
}

#[derive(Debug)]
pub enum Expr{
    Literal(String),
    Param(usize),
    FunctionCall(FuncDeclaration, Vec<Expr>),
}

#[derive(Debug)]
pub struct Vec2;

#[derive(Debug)]
pub enum BuiltinFunction{
    Shift(Vec2),
    Add,
    Mul,
    Div,
    Mod,
    Sub,
}

#[derive(Debug)]
pub enum DefinedFunction{
    Builtin(BuiltinFunction),
    UserDefined(Function),
}

#[derive(Debug)]
pub struct Function{
    pub params: Vec<usize>,
    pub body: Box<Expr>,
}

#[derive(Debug)]
pub enum FuncDeclaration {
    Lambda(Vec<Param>, Box<Expr>),
    Defined(DefinedFunction)
}

pub struct Scope {
    pub functions: Vec<FuncDeclaration>,
}

pub fn def_select_fun() ->Function {
    let params = vec![0,1,2,3];
    let body = Box::new(Expr::Param(0));

    Function{params, body}
}

pub fn def_user_fun() -> Function{
    
    let params = vec![0, 1];

    let body = Box::new(Expr::FunctionCall(FuncDeclaration::Defined(DefinedFunction::Builtin(BuiltinFunction::Add)), vec![Expr::Param(0), Expr::Param(1)]));

    Function{params, body}
}