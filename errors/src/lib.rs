use std::fmt;

use codespan::Span;

pub type Sp = Span;

#[derive(Debug)]
pub enum LexicalError {
    IsVeryBad,
    OpenComment(Sp),
    UnexpectedCharacter(Sp, char),
    OpenStringLiteral(Sp),
    LiteralIntOverflow(Sp),
    LiteralFloatOverflow(Sp),
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match *self {
            LexicalError::IsVeryBad => "izverybad",
            LexicalError::OpenComment(_) => "Not closed block comment",
            LexicalError::UnexpectedCharacter(_, _) => "Unexpected character",
            LexicalError::OpenStringLiteral(_) => "Not closed string literal",
            LexicalError::LiteralIntOverflow(_) => "Literal int overflowed",
            LexicalError::LiteralFloatOverflow(_) => "Literal float overflow",
        };
        val.fmt(f)
    }
}

#[derive(Debug)]
pub enum SemanticError {
    UndefinedIdentifier(Sp, String),
    UnusedArgument,
    // span of conflict, first is expected, second is provided
    TypeMismatch(Sp, String, String),
    OperationTypeMismatch(String, Sp, String, Sp),
    ArgumentsNumberMismatch,
    //UnknownFunction(Sp, String, Sp, Vec<String>),
    NotAssignedReturnVariable(Sp, String),
    VariableRedefinition(String, Sp, Sp),
    LogicTypeMismatch(String, String, Sp),
    UnknownFunction(String, Sp),
    ArgumentsMismatch(String, Sp, Vec<Vec<String>>),
    // type name, variable span, field name
    NoField(String, Sp, String),
}

#[derive(Debug)]
pub enum SemanticWarning {
    NotUsedArgument(Sp, String),
    UnusedVariable(Sp, String),
}
