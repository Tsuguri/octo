pub mod ast;
pub mod lexer;

#[macro_use]
extern crate lalrpop_util;

use lalrpop_util::ParseError;
use errors::{Lines, show_location};
lalrpop_mod!(pub grammar);


pub fn parse(location: &str, src: &str, lex: bool) -> Result<ast::Program, ()> {
    let lines = Lines::new(src);
    if lex {
        for lexeme in lexer::Lexer::new(src) {
            print!("{:?}, ", lexeme);
        }
        println!();
    }
    let lexer = lexer::Lexer::new(src);
    let result = grammar::ProgramParser::new().parse(lexer);
    match result {
        Result::Ok(ast) => Result::Ok(ast),
        Result::Err(error) => {
            show_error(location, &lines, error);
            Result::Err(())
        }
    }
}

fn show_error(
    filepath: &str,
    lines: &Lines,
    error: ParseError<usize, lexer::Token, errors::LexicalError>,
) {
    match error {
        ParseError::UnrecognizedToken { token, expected } => {
            show_unrecognized_token_error(filepath, lines, &token, expected)
        }

        ParseError::InvalidToken { location } => {
            show_invalid_token_error(filepath, lines, location)
        }
        ParseError::ExtraToken { token } => show_extra_token_error(filepath, lines, token),
        ParseError::User { error: err } => show_lexer_error(filepath, lines, err),
    }
}

// function borrowed from gluon (https://github.com/gluon-lang/gluon)
fn remove_extra_quotes(tokens: &mut [String]) {
    for token in tokens {
        if token.starts_with('"') && token.ends_with('"') {
            token.remove(0);
            token.pop();
        }
    }
}

fn show_lexer_error(filepath: &str, lines: &Lines, err: errors::LexicalError) {
    match err {
        errors::LexicalError::LiteralFloatOverflow(from, len) => show_location(
            filepath,
            lines,
            "Literal float overlow",
            (from, from + len - 1),
            Some("Consider changing this value"),
        ),
        errors::LexicalError::LiteralIntOverflow(from, len) => show_location(
            filepath,
            lines,
            "Literal int overflow",
            (from, from + len - 1),
            Some("Consider changing this value"),
        ),

        errors::LexicalError::OpenComment(location) => show_location(
            filepath,
            lines,
            "Parser found not closed comment",
            (location, location + 1),
            Some("Not closed comment starting here"),
        ),
        errors::LexicalError::OpenStringLiteral(location) => show_location(
            filepath,
            lines,
            "Parser found not closed string literal",
            (location, location),
            Some("Not closed literal starting here"),
        ),
        errors::LexicalError::UnexpectedCharacter(location, character) => show_location(
            filepath,
            lines,
            &format!("Parser found unexpected character: {}", character),
            (location, location),
            None,
        ),
        errors::LexicalError::IsVeryBad => panic!("Very bad error, please fill bug report"),
    }
}

fn show_invalid_token_error(filepath: &str, lines: &Lines, location: usize) {
    show_location(
        filepath,
        lines,
        &format!("Parser found invalid token."),
        (location, location),
        Some("Let me know if you see this as I've never encountered this error."),
    );
}

fn show_extra_token_error(
    filepath: &str,
    lines: &Lines,
    token: (usize, lexer::Token, usize),
) {
    show_location(
        filepath,
        lines,
        &format!("Parser found unexpected token: {}", token.1),
        (token.0, token.2),
        None,
    );
}

fn show_unrecognized_token_error(
    filepath: &str,
    lines: &Lines,
    token: &Option<(usize, lexer::Token, usize)>,
    mut expected: Vec<String>,
) {
    remove_extra_quotes(&mut expected);
    match token {
        None => println!("Unexpected end of file. Expected tokens: {:?}", expected),
        Some((start, token, end)) => {
            use crate::lexer::Token::*;
            let message = match token {
                BraceOpen => "unexpected block delimiter: '{'".to_owned(),
                BraceClose => "unexpected block delimiter: '}'".to_owned(),
                x => format!("Expected one of: {:?}, found \"{}\"", expected, x),
            };
            show_location(filepath, lines, &message, (*start, *end), None);
        }
    }
}
