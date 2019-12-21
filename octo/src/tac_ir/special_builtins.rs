use super::ir::{
    Address, 
    ConstantValue, 
    Operation, 
    StdFunction,
};
use super::code::Code;

#[derive(Debug)]
pub enum BuiltinEmitError {
    NameNotFound,
    CompilerError,
}