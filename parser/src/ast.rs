pub use codespan::ByteIndex;
pub use codespan::Span;
use crate::lexer::span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span<ByteIndex>,
    pub val: T,
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: Span<ByteIndex>) -> Spanned<T> {
        Spanned { span, val }
    }
    pub fn from_loc(val: T, from: usize, to: usize) -> Spanned<T> {
        Self::new(val, span(from, to))
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)?;
        Result::Ok(())
    }
}

#[derive(Debug)]
pub struct Program {
    pub items: Vec<GpuFunction>,
    pub pipeline: Pipeline,
}

#[derive(Debug, Clone)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Int,
    Bool,
    Void,
    Unknown,
}

impl PartialEq for Type {
    fn eq(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Float, Type::Float) => true,
            (Type::Int, Type::Int) => true,
            (Type::Bool, Type::Bool) => true,
            (Type::Vec2, Type::Vec2) => true,
            (Type::Vec3, Type::Vec3) => true,
            (Type::Vec4, Type::Vec4) => true,
            (_, _) => false,
        }
    }
}

impl Type {
    pub fn new(src: String) -> Type {
        match src.as_ref() {
            "float" => Type::Float,
            "int" => Type::Int,
            "bool" => Type::Bool,
            "vec2" => Type::Vec2,
            "vec3" => Type::Vec3,
            "vec4" => Type::Vec4,
            _ => Type::Unknown,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Type::*;
        write!(f, "{}", match self{
            Float => "float",
            Int => "int",
            Bool => "bool",
            Vec2 => "vec2",
            Vec3 => "vec3",
            Vec4 => "vec4",
            Void => "void",
            Unknown => "invalid_type",

        })?;
        Result::Ok(())
    }
}

//#[derive(Debug)]
//pub enum ProgramItem {
//    Function(Box<Function>),
//    GpuFunction(Box<GpuFunction>),
//}


#[derive(Debug)]
pub struct Pipeline {
    pub name: Spanned<String>,
    pub arguments: Vec<Variable>,
    pub results: Vec<Spanned<Type>>,
    pub block: Block,
}

//#[derive(Debug)]
//pub struct Function {
//    pub arguments: Vec<(Variable, Type)>,
//    pub name: String,
//    pub block: Block,
//    pub ret: Option<Type>,
//}
//
//impl Function {
//    pub fn new(name: String, arguments: Vec<(Variable, Type)>, block: Block) -> Function {
//        Function {
//            name,
//            arguments,
//            block,
//            ret: None,
//        }
//    }
//}

#[derive(Debug)]
pub struct GpuFunction {
    pub name: Spanned<String>,
    pub code: Spanned<String>,
    pub arguments: Vec<Variable>,
    pub results: Vec<Variable>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
//    Expression(Box<Expression>),
    Assignment(Box<Variable>, Box<Expression>, bool),
    Return(Box<Expression>),
}

#[derive(Debug)]
pub enum Expression {
    Variable(Variable),
//    Literal(Literal),
//    Negation(Box<Expression>),
//    Mul(Box<Expression>, Box<Expression>),
//    Div(Box<Expression>, Box<Expression>),
//    Add(Box<Expression>, Box<Expression>),
//    Sub(Box<Expression>, Box<Expression>),
//    Less(Box<Expression>, Box<Expression>),
//    LessEqual(Box<Expression>, Box<Expression>),
//    More(Box<Expression>, Box<Expression>),
//    MoreEqual(Box<Expression>, Box<Expression>),
//    Equals(Box<Expression>, Box<Expression>),
//    NotEquals(Box<Expression>, Box<Expression>),
//    And(Box<Expression>, Box<Expression>),
//    Or(Box<Expression>, Box<Expression>),
    Invocation(String, Vec<Box<Expression>>),
}

#[derive(Debug)]
pub enum Literal {
    Int(Spanned<i64>),
    Float(Spanned<f64>),
    String(Spanned<String>),
}

//#[derive(Debug)]
//pub struct Negated(Expression);
//
#[derive(Debug, Clone)]
pub struct Variable {
    pub identifier: Spanned<String>,
    pub typ: Type,
}

impl Variable {
    pub fn new(identifier: Spanned<String>) -> Variable {
        Variable {
            identifier,
            typ: Type::Unknown,
        }
    }

    pub fn typed(identifier: Spanned<String>, typ: Type) -> Variable {
        Variable { identifier, typ }
    }
}
