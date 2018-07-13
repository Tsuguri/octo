use octo_parser::ast;

pub enum SemanticError {
    UndefinedIdentifier,
    UnusedArgument,
    TypeMismatch,
    ArgumentsNumberMismatch,
}

pub struct Scope {
    //
}

pub struct Env {}
