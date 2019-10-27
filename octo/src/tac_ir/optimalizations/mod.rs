
use super::ir as ir;
use super::code as code;

mod remove_unused;
mod constants_propagation;

pub use constants_propagation::propagate_constants;
pub use remove_unused::remove_unused_operations;