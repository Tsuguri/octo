use std::fmt;
use std::str::CharIndices;

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),

    And,          //
    Or,           //
    If,           //
    Else,         //
    For,          //
    True,          //
    False,          //
    ParOpen,      //
    ParClose,     //
    Colon,        //
    Semicolon,    //
    Slash,        //
    Comma,        //
    Dot,          //
    BraceOpen,    //
    BraceClose,   //
    BracketOpen,  //
    BracketClose, //
    Question,     //
    ExclMark,     //
    Star,         //
    Plus,         //
    Minus,        //
    NotEqual,     //
    VeryEqual,    //
    Equal,        //
    Greater,      //
    GreaterEqual, //
    Less,         //
    LessEqual,    //
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Token::*;
        let val = match *self {
            Identifier(ref x) => "Identifier(".to_owned() + x + ")",
            StringLiteral(ref x) => "StringLiteral(".to_owned() + &x + ")",
            IntLiteral(ref x) => "IntLiteral(".to_owned() + &x.to_string() + ")",
            FloatLiteral(ref x) => "FloatLiteral(".to_owned() + &x.to_string() + ")",
            And => "And".to_owned(),
            Or => "Or".to_owned(),
            If => "If".to_owned(),
            Else => "Else".to_owned(),
            For => "For".to_owned(),
            True => "True".to_owned(),
            False => "False".to_owned(),
            ParOpen => "ParOpen".to_owned(),
            ParClose => "ParClose".to_owned(),
            Colon => "Colon".to_owned(),
            Semicolon => "Semicolon".to_owned(),
            Slash => "Slash".to_owned(),
            Comma => "Comma".to_owned(),
            Dot => "Dot".to_owned(),
            BraceOpen => "BraceOpen".to_owned(),
            BraceClose => "BraceClose".to_owned(),
            BracketOpen => "BracketOpen".to_owned(),
            BracketClose => "BracketClose".to_owned(),
            Question => "QuestionMark".to_owned(),
            ExclMark => "ExclamationMark".to_owned(),
            Star => "Star".to_owned(),
            Plus => "Plus".to_owned(),
            Minus => "Minus".to_owned(),
            NotEqual => "NotEqual".to_owned(),
            VeryEqual => "VeryEqual".to_owned(),
            Equal => "Equal".to_owned(),
            Greater => "Greater".to_owned(),
            GreaterEqual => "GreaterEqual".to_owned(),
            Less => "Less".to_owned(),
            LessEqual => "LessEqual".to_owned(),
        };
        val.fmt(f)
    }
}
#[derive(Debug)]
pub enum LexicalError {
    IzBad,
    IsVeryBad,
    OpenComment(usize),
    UnexpectedCharacter(usize, char),
    OpenStringLiteral(usize),
    LiteralIntOverflow(usize),
    LiteralFloatOverflow(usize)
}
impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match *self {
            LexicalError::IzBad => "isbad",
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

pub struct Lexer<'input> {
    _source: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    current_line: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();
        let first = chars.next();
        Lexer {
            _source: input,
            chars: chars,
            lookahead: first,
            current_line: 1, // human friendly line counting (not 0-indexed :)
        }
    }

    fn pop(&mut self) -> Option<(usize, char)> {
        match self.lookahead {
            Some((a, b)) => {
                self.lookahead = self.chars.next();
                if b == '\n' {
                    // I'm not very confident in this.
                    // Nothing should happen after you pop newline and before you pop anything else.
                    self.current_line += 1;
                }
                Some((a, b))
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<char> {
        match self.lookahead {
            Some((_, y)) => Some(y),
            None => None,
        }
    }

    fn test(&self, c: char) -> bool {
        match self.lookahead {
            None => false,
            Some((_, x)) => c == x,
        }
    }

    fn test_digit(&self) -> bool {
        match self.lookahead {
            None => false,
            Some((_, x)) => x.is_digit(10),
        }
    }

    fn test_alphabetic(&self) -> bool {
        match self.lookahead {
            None => false,
            Some((_, x)) => x.is_alphabetic(),
        }
    }

    fn test_alphanumeric(&self) -> bool {
        match self.lookahead {
            None => false,
            Some((_, x)) => x.is_alphanumeric(),
        }
    }

    fn read_string_literal(&mut self, start: usize) -> Result<String, LexicalError> {
        let mut string = String::new();
        while let Some((_, x)) = self.pop() {
            match x {
                '"' => return Result::Ok(string),
                x => string.push(x),
            }
        }

        Result::Err(LexicalError::OpenStringLiteral(start))
    }

    fn read_number(&mut self, first: char, start: usize) -> Result<Token, LexicalError> {
        let mut string = String::new();
        string.push(first);
        while self.test_digit() {
            match self.pop() {
                None => unreachable!(),
                Some((_, x)) => string.push(x),
            }
        }
        let is_float = self.test('.');
        if is_float {
            string.push('.');
            self.pop();
            while self.test_digit() {
                match self.pop() {
                    None => unreachable!(),
                    Some((_, x)) => string.push(x),
                }
            }
            
        }
        if !is_float {
            match string.parse::<i64>() {
                Result::Ok(literal) => Result::Ok(Token::IntLiteral(literal)),
                Result::Err(_) => Result::Err(LexicalError::LiteralIntOverflow(start)),
            }
        } else {
            match string.parse::<f64>() {
                Result::Ok(literal) => Result::Ok(Token::FloatLiteral(literal)),
                Result::Err(_) => Result::Err(LexicalError::IsVeryBad), 
            }
        }
    }

    fn read_identifier(&mut self, first: char, start: usize) -> Result<String, LexicalError> {
        let mut string = String::new();
        string.push(first);
        while self.test_alphanumeric() {
            match self.pop() {
                None => unreachable!(),
                Some((_, x)) => string.push(x),
            }
        }
        return Result::Ok(string);

    }
    fn remove_block_comment(&mut self) -> Result<(), LexicalError> {
        let mut opened_blocks = 1u32;

        let start = match self.pop() {
            None => return Result::Err(LexicalError::IsVeryBad),
            Some((x, _)) => x,
        };
        while let Some((_, x)) = self.pop() {
            match x {
                '/' => {
                    if self.test('*') {
                        self.pop();
                        opened_blocks += 1;
                    }
                }
                '*' => {
                    if self.test('/') {
                        self.pop();
                        opened_blocks -= 1;
                        if opened_blocks == 0 {
                            return Result::Ok(());
                        }
                    }
                }
                _ => (),
            }
        }
        Result::Err(LexicalError::OpenComment(start - 1))
    }
}
macro_rules! ok_m {
    ($x:ident, $y:expr, $l:expr) => {
        Some(Ok(($y, Token::$x, $y + $l)))
    };
}

macro_rules! ok {
    ($x:ident, $y:expr) => {
        ok_m!($x, $y, 1)
    };
}
macro_rules! err {
    ($x:expr) => {
        Some(Result::Err($x))
    };
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<(usize, Token, usize), LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.pop() {
                None => return None, // end of tokens

                Some((_, ' ')) => continue,  // skip whitespace characters
                Some((_, '\t')) => continue, //
                Some((_, '\n')) => continue, //

                Some((i, '(')) => return ok!(ParOpen, i),
                Some((i, ')')) => return ok!(ParClose, i),
                Some((i, ';')) => return ok!(Semicolon, i),
                Some((i, '.')) => return ok!(Dot, i),
                Some((i, ':')) => return ok!(Colon, i),
                Some((i, '[')) => return ok!(BracketOpen, i),
                Some((i, ']')) => return ok!(BracketClose, i),
                Some((i, '{')) => return ok!(BraceOpen, i),
                Some((i, '}')) => return ok!(BraceClose, i),
                Some((i, '?')) => return ok!(Question, i),
                Some((i, '*')) => return ok!(Star, i),
                Some((i, '+')) => return ok!(Plus, i),
                Some((i, '-')) => return ok!(Minus, i),
                Some((i, '!')) => match self.peek() {
                    Some('=') => {
                        self.pop();
                        return ok_m!(NotEqual, i, 2);
                    }
                    _ => return ok!(ExclMark, i),
                },
                Some((i, '<')) => match self.peek() {
                    Some('=') => {
                        self.pop();
                        return ok_m!(LessEqual, i, 2);
                    }
                    _ => return ok!(Less, i),
                },
                Some((i, '>')) => match self.peek() {
                    Some('=') => {
                        self.pop();
                        return ok_m!(GreaterEqual, i, 2);
                    }
                    _ => return ok!(Greater, i),
                },
                Some((i, '=')) => match self.peek() {
                    Some('=') => {
                        self.pop();
                        return ok_m!(VeryEqual, i, 2);
                    }
                    _ => return ok!(Equal, i),
                },

                Some((i, '/')) => match self.peek() {
                    Some('/') => {
                        while !self.test('\n') {}
                        continue;
                    }
                    Some('*') => match self.remove_block_comment() {
                        Result::Ok(()) => continue,
                        Result::Err(er) => return err!(er),
                    },
                    _ => return ok!(Slash, i), // next character is whatever so we emit normal slash
                },
                Some((i, '\"')) => match self.read_string_literal(i) {
                    Result::Ok(slice) => {
                        let len = slice.len();
                        return Some(Result::Ok((i, Token::StringLiteral(slice), i + len + 2))); // 2 because of two "s.
                    }
                    Result::Err(err) => return err!(err),
                },
                Some((i, ch)) if ch.is_digit(10) => match self.read_number(ch, i) {
                    Result::Ok(result) => return Some(Result::Ok((i, result, i + 1))),
                    Result::Err(err) => return err!(err),
                },
                Some((i,ch)) if ch.is_alphabetic() => match self.read_identifier(ch, i) {
                    Result::Ok(result) => {
                        let len = result.len();
                        // TODO: change into keywords dictionary
                        match result.as_ref() {
                            "for" => return ok_m!(For, i, i + 3),
                            "if" => return ok_m!(If, i, i + 2),
                            "else" => return ok_m!(Else, i, i + 4),
                            "and" => return ok_m!(And, i, i + 3),
                            "or" => return ok_m!(Or, i, i + 2),
                            "true" => return ok_m!(True, i, i + 4),
                            "false" => return ok_m!(False, i, i + 5),
                            x => return Some(Result::Ok((i, Token::Identifier(x.to_owned()), i + len))),
                        }
                    }
                    Result::Err(err) => return err!(err),
                }
                Some((i, c)) => return err!(LexicalError::UnexpectedCharacter(i, c)),
            }
        }
    }
}
