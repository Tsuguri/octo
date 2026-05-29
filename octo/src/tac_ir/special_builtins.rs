use super::code::Code;
use super::ir::{Address, ConstantValue, Operation, StdFunction};

#[derive(Debug)]
pub enum BuiltinEmitError {
    NameNotFound,
    CompilerError,
}
