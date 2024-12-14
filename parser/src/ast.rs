use crate::lexer::span;
pub use codespan::ByteIndex;
pub use codespan::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub span: Span,
    pub val: T,
}

impl<T> Spanned<T> {
    pub fn new(val: T, span: Span) -> Spanned<T> {
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

#[derive(Debug, Clone, Copy, Hash, Eq)]
pub enum Type {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat3,
    Mat4,
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
            (Type::Mat3, Type::Mat3) => true,
            (Type::Mat4, Type::Mat4) => true,
            (Type::Void, Type::Void) => true,
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
            "mat3" => Type::Mat3,
            "mat4" => Type::Mat4,
            _ => Type::Unknown,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Type::*;
        write!(
            f,
            "{}",
            match self {
                Float => "float",
                Int => "int",
                Bool => "bool",
                Vec2 => "vec2",
                Vec3 => "vec3",
                Vec4 => "vec4",
                Mat3 => "mat3",
                Mat4 => "mat4",
                Void => "void",
                Unknown => "invalid_type",
            }
        )?;
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
    pub uniforms: Option<UniformBlock>,
}

#[derive(Debug)]
pub struct UniformBlock {
    pub entries: Vec<Variable>,
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum ValueStorage {
    Existing(Vec<Spanned<String>>),
    Creation(Spanned<String>),
}

#[derive(Debug)]
pub enum Statement {
    Expression(Box<Expression>),
    Assignment(ValueStorage, Box<Expression>),
    Return(Box<Expression>),
    For(Box<Statement>, Box<Expression>, Box<Statement>, Block),
    IfElse(Box<Expression>, Block, Option<Block>),
}

pub fn extract_access_chain(exp: Box<Expression>) -> Result<Vec<Spanned<String>>, ()> {
    let mut exp = *exp;
    match exp {
        Expression::Access(exp2, field) => {
            let mut path = extract_access_chain(exp2)?;
            path.push(field);
            Result::Ok(path)
        }
        Expression::Variable(var) => Result::Ok(vec![var.identifier]),
        _ => Result::Err(()),
    }
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
    Shift(Box<Expression>, Box<Expression>),
    Scale(Box<Expression>, Box<Expression>),
    Invocation(Spanned<String>, Vec<Box<Expression>>),
    Access(Box<Expression>, Spanned<String>),
}

fn concat_spans(span1: Span, span2: Span) -> Span {
    Span::new(span1.start(), span2.end())
}

impl Expression {
    pub fn span(&self) -> Span {
        use Expression::*;
        match self {
            Expression::Variable(var) => var.identifier.span,
            Expression::Literal(lit) => lit.span(),
            Negation(left) => left.span(),
            Mul(left, right) => concat_spans(left.span(), right.span()),
            Div(left, right) => concat_spans(left.span(), right.span()),
            Add(left, right) => concat_spans(left.span(), right.span()),
            Sub(left, right) => concat_spans(left.span(), right.span()),
            Less(left, right) => concat_spans(left.span(), right.span()),
            LessEqual(left, right) => concat_spans(left.span(), right.span()),
            More(left, right) => concat_spans(left.span(), right.span()),
            MoreEqual(left, right) => concat_spans(left.span(), right.span()),
            Equals(left, right) => concat_spans(left.span(), right.span()),
            NotEquals(left, right) => concat_spans(left.span(), right.span()),
            And(left, right) => concat_spans(left.span(), right.span()),
            Or(left, right) => concat_spans(left.span(), right.span()),
            // TODO: fix shift and scale. These are not including parentheses and Shift/Scale keyword
            Shift(left, right) => concat_spans(left.span(), right.span()),
            Scale(left, right) => concat_spans(left.span(), right.span()),
            _ => span(0, 1),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Int(Spanned<i64>),
    Float(Spanned<f64>),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::Int(x) => x.span,
            Literal::Float(x) => x.span,
        }
    }
}

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
