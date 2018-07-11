#[derive(Debug)]
pub struct Statement {
    pub value: i64,
}
#[derive(Debug)]
pub struct Program {
    pub lines: Vec<Primitive>,
}

#[derive(Debug)]
pub enum Primitive {
    Variable(Box<Variable>),
    Literal(Box<Literal>),
    Negation(Box<Primitive>),
    Mul(Box<Primitive>, Box<Primitive>),
    Div(Box<Primitive>, Box<Primitive>),
    Add(Box<Primitive>, Box<Primitive>),
    Sub(Box<Primitive>, Box<Primitive>),
    Less(Box<Primitive>, Box<Primitive>),
    LessEqual(Box<Primitive>, Box<Primitive>),
    More(Box<Primitive>, Box<Primitive>),
    MoreEqual(Box<Primitive>, Box<Primitive>),
    Equals(Box<Primitive>, Box<Primitive>),
    NotEquals(Box<Primitive>, Box<Primitive>),
    And(Box<Primitive>, Box<Primitive>),
    Or(Box<Primitive>, Box<Primitive>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub struct Negated(Primitive);

#[derive(Debug)]
pub struct Variable {
    pub identifier: String,
}
