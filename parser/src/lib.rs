#[macro_use]
extern crate lalrpop_util;

use std::convert::Into;

pub use codespan;
use codespan::CodeMap;
pub use codespan_reporting;
use codespan_reporting::Diagnostic;
use lalrpop_util::ParseError;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub grammar);

type ParseErr = ParseError<usize, lexer::Token, errors::LexicalError>;

pub struct FailedParsing {
    pub errors: Vec<ParseErr>,
}

pub fn parse(src: &str, lex: bool) -> Result<ast::Pipeline, FailedParsing> {
    if lex {
        for lexeme in lexer::Lexer::new(src) {
            print!("{:?}, ", lexeme);
        }
        println!();
    }
    let lexer = lexer::Lexer::new(src);
    let result = grammar::PipelineParser::new().parse(lexer);
    match result {
        Result::Ok(ast) => Result::Ok(ast),
        Result::Err(error) => Result::Err(FailedParsing {
            errors: vec![error],
        }),
    }
}

pub struct ErrWrap<'a> {
    pub err: &'a ParseErr,
}

macro_rules! error {
    ($message:expr; $from:expr, $to: expr) => {
        error!($message, lexer::span($from, $to))
    };
    ($message:expr, $span:expr) => {
        Diagnostic::new_error($message).with_label(codespan_reporting::Label::new_primary($span))
    };
    ($message:expr, $span:expr, $label:expr) => {
        Diagnostic::new_error($message)
            .with_label(codespan_reporting::Label::new_primary($span).with_message($label))
    };
}

impl<'a> From<ErrWrap<'a>> for Diagnostic {
    fn from(w: ErrWrap) -> Diagnostic {
        match &w.err {
            ParseError::UnrecognizedToken {
                token,
                expected: exp,
            } => {
                let mut expected = exp.clone();
                remove_extra_quotes(&mut expected);
                match token {
                    None => Diagnostic::new_error(format!(
                        "Unexpected end of file. Expected one of: {:?}",
                        expected
                    )),
                    Some((start, token, end)) => {
                        use crate::lexer::Token::*;
                        let message = match token {
                            BraceOpen => "unexpected block delimiter: '{'".to_owned(),
                            BraceClose => "unexpected block delimiter: '}'".to_owned(),
                            x => format!("Expected one of: {:?}, found \"{}\"", expected, x),
                        };
                        Diagnostic::new_error(message).with_label(
                            codespan_reporting::Label::new_primary(lexer::span(*start, *end + 1))
                                .with_message("Problem occurs here"),
                        )
                    }
                }
            }
            ParseError::InvalidToken { location } => error!("Parser found invalid token."; *location, *location + 1),
            ParseError::ExtraToken { token } => error!(format!("Parser found unexpected token: {}", token.1);token.0, token.2),
            ParseError::User { error } => match error {
                errors::LexicalError::LiteralFloatOverflow(span) => error!(
                    "Float literal overflow",
                    *span, "Consider changing this value"
                ),

                errors::LexicalError::LiteralIntOverflow(span) => error!(
                    "Int literal overflow",
                    *span, "Consider changing this value"
                ),

                errors::LexicalError::OpenComment(span) => error!(
                    "Unclosed comment",
                    *span, "Not closed comment starting here"
                ),

                errors::LexicalError::IsVeryBad => panic!("Very bad error, please fill bug report"),
                errors::LexicalError::OpenStringLiteral(span) => error!(
                    "Parser found not closed string literal",
                    *span, "Not closed literal starts here."
                ),

                errors::LexicalError::UnexpectedCharacter(span, character) => error!(
                    format!("Parser found unexpected character: {}", character),
                    *span
                ),
            },
        }
    }
}

// function borrowed from gluon (https://github.com/gluon-lang/gluon)
fn remove_extra_quotes(tokens: &mut [String]) {
    for token in tokens {
        if token.starts_with('"') & &token.ends_with('"') {
            token.remove(0);
            token.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fast_expression(code: &str) -> bool {
        let lexer = lexer::Lexer::new(code);
        let result = grammar::ExpressionParser::new().parse(lexer);
        result.is_ok()
    }

    fn fast_statement(code: &str) -> bool {
        let lexer = lexer::Lexer::new(code);
        let result = grammar::StatementParser::new().parse(lexer);
        result.is_ok()
    }

    fn fast_block(code: &str) -> bool {
        let lexer = lexer::Lexer::new(code);
        let result = grammar::BlockParser::new().parse(lexer);
        result.is_ok()
    }

    fn fast_op(code: &str) -> bool {
        let lexer = lexer::Lexer::new(code);
        let result = grammar::OpParser::new().parse(lexer);
        result.is_ok()
    }

    #[test]
    fn acceptance_test() {
        // to be done later once I know how to extract parsers for single non-terminals

        assert!(fast_expression("2"));
        assert!(fast_expression("2+2"));
        assert!(fast_expression("2-2"));
        assert!(fast_expression("2*3"));
        assert!(fast_expression("2/3"));

        assert!(fast_statement("let p = 2+2"));
        assert!(fast_statement("dwa = 2 + 2"));
        assert!(fast_statement("return 3.0"));
        assert!(fast_statement("dwa = 2 + 2"));

        // well, it might be valid expression
        assert!(fast_op("for(i;i;i){}"));
        assert!(fast_op("if(1){}"));

        assert!(fast_block("{for(i;i;i){}}"));
        assert!(fast_block("{if(i){}}"));
        assert!(fast_block("{if(i){}else {}}"));

        //
        //        assert!(result.is_ok());
        //        let result = result.unwrap();
        //        match result {
        //            ast::Literal::Int(val) =>{
        //                assert!(val.val == 2);
        //            },
        //            _ => assert!(false),
        //        }
    }
}
