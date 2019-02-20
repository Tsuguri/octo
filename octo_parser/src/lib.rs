pub mod ast;
pub mod grammar;
pub mod lexer;

extern crate lalrpop_util;

use lalrpop_util::ParseError;

#[derive(Debug)]
pub struct ParsingError {
    what: String,
}

#[derive(Debug)]
pub struct Errors(Vec<ParsingError>);

#[derive(Debug)]
struct Lines {
    offsets: Vec<usize>,
    end: usize,
}

impl Lines {
    pub fn new(src: &str) -> Lines {
        let mut offsets = vec![];
        let src_len = src.len();
        offsets.push(0);
        let mut last = 0;
        for line in src.split('\n') {
            let len = line.len();
            offsets.push(len + last + 1);
            last = len + last + 1;
        }
        let _lines_count = offsets.len();
        // if lines_count > 1 {
        //     if offsets[lines_count - 1] == offsets[lines_count - 2] + 1 {
        //         offsets.pop();
        //     }
        // }
        //println!("{:?}", offsets);
        Lines {
            offsets,
            end: src_len,
        }
    }

    pub fn lines(&self) -> usize {
        self.offsets.len() - 1
    }

    pub fn get_line(&self, position: usize) -> Option<(usize, usize, usize)> {
        let line = self.offsets.iter().position(|x| x > &position);
        match line {
            None => Option::None,
            Some(i) => Option::Some((i - 1, self.offsets[i - 1], self.offsets[i])), // i will be always at least 1
        }
    }
    pub fn get_line_span(&self, line: usize) -> Option<(usize, usize)> {
        match line {
            x if x >= self.lines() => None, // invalid line (after EOF)
            x => Some((self.offsets[x], self.offsets[x + 1] - 1)),
        }
    }
}

pub fn parse(location: &str, src: &str, lex: bool) -> Result<ast::Program, ()> {
    let lines = Lines::new(src);
    if lex {
        for lexem in lexer::Lexer::new(src) {
            print!("{:?}, ", lexem);
        }
        println!();
    }
    let lexer = lexer::Lexer::new(src);
    let result = grammar::ProgramParser::new().parse(lexer);
    match result {
        Result::Ok(ast) => Result::Ok(ast),
        Result::Err(error) => {
            show_error(location, src, &lines, error);
            Result::Err(())
        }
    }
}

fn show_error(
    filepath: &str,
    src: &str,
    lines: &Lines,
    error: ParseError<usize, lexer::Token, lexer::LexicalError>,
) {
    match error {
        ParseError::UnrecognizedToken { token, expected } => {
            show_unrecognized_token_error(filepath, lines, src, &token, expected)
        }

        ParseError::InvalidToken { location } => {
            show_invalid_token_error(src, filepath, lines, location)
        }
        ParseError::ExtraToken { token } => show_extra_token_error(filepath, src, lines, token),
        ParseError::User { error: err } => show_lexer_error(filepath, src, lines, err),
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

fn show_code_snippet(
    src: &str,
    line: usize,
    lines: &Lines,
    underscore: (usize, usize),
    help: Option<&str>,
) {
    //
    //println!("Entire code lenght: {}", src.len());
    let snippet = match line {
        0 => [0, 1, 2],                                   // first line
        n if n + 1 == lines.lines() => [n - 2, n - 1, n], // last line
        n => [n - 1, n, n + 1],                           // all others
    };
    for snip in snippet.iter() {
        let (from, to) = lines.get_line_span(*snip).unwrap();
        // println!(
        //     "start: {}, end: {}, from: {}, to: {}",
        //     underscore.0, underscore.1, from, to
        // );
        let content = &src[from..to];
        print!("{:4}|", snip + 1);
        println!("{}", content);
        if from <= underscore.0 && to >= underscore.1 {
            let spaces = " ".repeat(5 + underscore.0 - from);
            print!("{}", spaces);
            let underscore = "^".repeat(underscore.1 - underscore.0 + 1);
            print!("{}", underscore);
            if let Some(message) = help {
                print!(" {}", message);
            }
            println!("");
        }
    }
}

fn show_location(
    filepath: &str,
    src: &str,
    lines: &Lines,
    message: &str,
    location: (usize, usize),
    help: Option<&str>,
) {
    println!("error: {}", message);
    match lines.get_line(location.0) {
        None => println!("in file: {} at unknown position", filepath),
        Some((line, _, _)) => {
            println!("--> at: {},{}", filepath, line);
            show_code_snippet(src, line, lines, (location.0, location.1), help);
        }
    }
}

fn show_lexer_error(filepath: &str, src: &str, lines: &Lines, err: lexer::LexicalError) {
    match err {
        lexer::LexicalError::LiteralFloatOverflow(from, len) => show_location(
            filepath,
            src,
            lines,
            "Literal float overlow",
            (from, from + len - 1),
            Some("Consider changing this value"),
        ),
        lexer::LexicalError::LiteralIntOverflow(from, len) => show_location(
            filepath,
            src,
            lines,
            "Literal int overflow",
            (from, from + len - 1),
            Some("Consider changing this value"),
        ),

        lexer::LexicalError::OpenComment(location) => show_location(
            filepath,
            src,
            lines,
            "Parser found not closed comment",
            (location, location + 1),
            Some("Not closed comment starting here"),
        ),
        lexer::LexicalError::OpenStringLiteral(location) => show_location(
            filepath,
            src,
            lines,
            "Parser found not closed string literal",
            (location, location),
            Some("Not closed literal starting here"),
        ),
        lexer::LexicalError::UnexpectedCharacter(location, character) => show_location(
            filepath,
            src,
            lines,
            &format!("Parser found unexpected character: {}", character),
            (location, location),
            None,
        ),
        lexer::LexicalError::IsVeryBad => panic!("Very bad error, please fill bug report"),
    }
}

fn show_invalid_token_error(filepath: &str, src: &str, lines: &Lines, location: usize) {
    show_location(
        filepath,
        src,
        lines,
        &format!("Parser found invalid token."),
        (location, location),
        Some("Let me know if you see this as I've never encountered this error."),
    );
}

fn show_extra_token_error(
    filepath: &str,
    src: &str,
    lines: &Lines,
    token: (usize, lexer::Token, usize),
) {
    show_location(
        filepath,
        src,
        lines,
        &format!("Parser found unexpected token: {}", token.1),
        (token.0, token.2),
        None,
    );
}

fn show_unrecognized_token_error(
    filepath: &str,
    lines: &Lines,
    src: &str,
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
            show_location(filepath, src, lines, &message, (*start, *end), None);
        }
    }
}
