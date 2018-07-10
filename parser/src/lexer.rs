use std::str::CharIndices;
use std::fmt;

#[derive(Debug)]
pub enum Token<'input> {
    Identifier(&'input str),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    

    And,
    Or,
    If,
    Else,
    For,
    ParOpen,
    ParClose,
    Colon,
    Semicolon,
    Comma,
    Dot,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Question,
    ExclMark,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        let val = match *self {
            Identifier(ref x) => "Identifier(".to_owned()+x+")",
            StringLiteral(ref x) => "StringLiteral(".to_owned()+&x+")",
            IntLiteral(ref x) => "IntLiteral(".to_owned()+&x.to_string()+")",
            FloatLiteral(ref x) => "FloatLiteral(".to_owned()+&x.to_string()+")",
            And => "And".to_owned(),
            Or=> "Or".to_owned(),
            If=> "If".to_owned(),
            Else=> "Else".to_owned(),
            For=>  "For".to_owned(),
            ParOpen=> "ParOpen".to_owned(),
            ParClose=> "ParClose".to_owned(),
            Colon=> "Colon".to_owned(),
            Semicolon=> "Semicolon".to_owned(),
            Comma=> "Comma".to_owned(),
            Dot=> "Dot".to_owned(),
            BraceOpen=> "BraceOpen".to_owned(),
            BraceClose=> "BraceClose".to_owned(),
            BracketOpen=> "BracketOpen".to_owned(),
            BracketClose=> "BracketClose".to_owned(),
            Question=> "QuestionMark".to_owned(),
            ExclMark=> "ExclamationMark".to_owned(),
        };
        val.fmt(f)


    }

}
#[derive(Debug)]
pub enum LexicalError {
    IzBad,
    IsVeryBad
}
impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match *self {
            LexicalError::IzBad => "isbad",
            LexicalError::IsVeryBad => "izverybad",
        };
        val.fmt(f)
    }
}


pub struct Lexer<'input> {
    _source: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,

}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let first = chars.next();
        Lexer { _source: input, chars: chars, lookahead: first }
    }

    fn pop(&mut self) -> Option<(usize,char)>{
        match self.lookahead {
            Some(a) =>{
                self.lookahead = self.chars.next();
                Some(a)
            }
            None => None
        }
    }

    fn test<F>(&self, test: F) -> bool where F: Fn(char)->bool {
        self.lookahead.map_or(false, |(_, ch)| test(ch))
    }
}

macro_rules! ok {
    ($x:ident, $y:expr) => (
        Some(Ok(($y, Token::$x, $y+1)))
)}


impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token<'input>, usize), LexicalError>;

    fn next(&mut self) ->Option<Self::Item> {
        loop {
            match self.pop() {
                Some((i, '(')) => return ok!(ParOpen, i),
                Some((i, ')')) => return ok!(ParClose, i),
                Some((i, ';')) => return ok!(Semicolon, i),
                Some((i, ':')) => return ok!(Colon, i),
                Some((i, '[')) => return ok!(BracketOpen, i),
                Some((i, ']')) => return ok!(BracketClose, i),
                Some((i, '{')) => return ok!(BraceOpen, i),
                Some((i, '}')) => return ok!(BraceClose, i),
                Some((i, '?')) => return ok!(Question, i),
                Some((i, '!')) => return ok!(ExclMark, i),
                None => return None,
                _ => continue,
            }
        }
    }
}
