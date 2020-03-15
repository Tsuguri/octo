use super::code;
use super::ir;

mod constants_propagation;
mod remove_unused;
mod unroll_loop;

pub use constants_propagation::propagate_constants;
pub use remove_unused::remove_unused_operations;
pub use unroll_loop::unroll_synced_loop;
