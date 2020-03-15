mod for_loop;
mod peekable_code;
mod if_else;

pub use peekable_code::PeekableCode;
pub use for_loop::{LoopCode,find_loop};
pub use if_else::{find_if_else, IfElseCode};
