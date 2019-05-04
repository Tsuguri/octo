#[derive(Debug)]
pub struct Program {
    pub items: Vec<ProgramItem>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Float,
    Int,
    Bool,
    String,
    Void,
    Unknown,
    UserDefined(String),
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Float, Type::Float) => true,
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::String, Type::String) => true,
            (Type::UserDefined(x), Type::UserDefined(y)) if x == y => true,
            (_, _) => false,
        }
    }
}

impl Type {
    pub fn new(src: String) -> Type {
        match src.as_ref() {
            "float" => return Type::Float,
            "int" => return Type::Int,
            "string" => return Type::String,
            "bool" => return Type::Bool,
            _ => (),
        };
        Type::UserDefined(src)
    }
}

#[derive(Debug)]
pub enum ProgramItem {
    Function(Box<Function>),
    GpuFunction(Box<GpuFunction>),
}

#[derive(Debug)]
pub struct Function {
    pub arguments: Vec<(Variable, Type)>,
    pub name: String,
    pub block: Block,
    pub ret: Option<Type>,
}

impl Function {
    pub fn new(name: String, arguments: Vec<(Variable, Type)>, block: Block) -> Function {
        Function {
            name,
            arguments,
            block,
            ret: None,
        }
    }
}

#[derive(Debug)]
pub struct GpuFunction {
    pub name: String,
    pub code: String,
    pub arguments: Vec<(Variable, Type)>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Assignment(Box<Variable>, Box<Expression>, bool),
}

#[derive(Debug)]
pub enum Expression {
    Variable(Variable),
    Literal(Literal),
    Negation(Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    More(Box<Expression>, Box<Expression>),
    MoreEqual(Box<Expression>, Box<Expression>),
    Equals(Box<Expression>, Box<Expression>),
    NotEquals(Box<Expression>, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Invocation(String, Vec<Box<Expression>>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub struct Negated(Expression);

#[derive(Debug)]
pub struct Variable {
    pub identifier: String,
    pub typ: Type,
}

impl Variable {
    pub fn new(identifier: String) -> Variable {
        Variable {
            identifier,
            typ: Type::Unknown,
        }
    }

    pub fn typed(identifier: String, typ: Type) -> Variable {
        Variable { identifier, typ }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn firt_test() {
        assert_eq!(1, 1);
    }
}
