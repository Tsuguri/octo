use std::fmt;

use codespan::Span;

pub type Sp = Span<codespan::ByteIndex>;

pub fn helpo() {
    println!("heh");
}
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
