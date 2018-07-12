#[derive(Debug)]
pub struct Program {
    pub items: Vec<ProgramItem>,
}

#[derive(Debug)]
pub enum ProgramItem {
    Function(Box<Function>),
    GpuFunction(Box<GpuFunction>),
}

#[derive(Debug)]
pub struct Function {
    pub arguments: Vec<(Variable, String)>,
    pub name: String,
    pub block: Block,
}

#[derive(Debug)]
pub struct GpuFunction {
    pub name: String,
    pub code: String,
    pub arguments: Vec<(Variable, String)>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Assignment(Box<Variable>, Expression),
}

#[derive(Debug)]
pub enum Expression {
    Variable(Box<Variable>),
    Literal(Box<Literal>),
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
}
