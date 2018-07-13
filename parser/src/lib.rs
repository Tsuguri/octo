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
        let lines_count = offsets.len();
        if lines_count > 1 {
            if offsets[lines_count - 1] == offsets[lines_count - 2] + 1 {
                offsets.pop();
            }
        }
        Lines {
            offsets,
            end: src_len,
        }
    }

    pub fn lines(&self) -> usize {
        self.offsets.len()
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
            x if x >= self.offsets.len() => None, // invalid line (after EOF)
            x => Some((self.offsets[line], self.offsets[line + 1])),
        }
    }
}

pub fn parse(location: &str, src: &str, lex: bool) -> Result<ast::Program, Errors> {
    let lines = Lines::new(src);
    if (lex) {
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
            Result::Err(Errors(vec![]))
        }
    }
}

fn show_error(
    location: &str,
    src: &str,
    lines: &Lines,
    error: ParseError<usize, lexer::Token, lexer::LexicalError>,
) {
    match error {
        ParseError::UnrecognizedToken { token, expected } => {
            show_unrecognized_token_error(location, &lines, src, &token, expected)
        }
        ParseError::InvalidToken { location: _ } => println!("heh, invalid"),
        ParseError::ExtraToken { token: _ } => {
            println!("heh, extra");
            // additional, unexpected tokens
        }
        ParseError::User { error: err } => {
            show_lexer_error(err);
        }
    }
}

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
    let snippet = match line {
        0 => [0, 1, 2],
        n => [n - 1, n, n + 1],
    };
    for snip in snippet.iter() {
        let (from, to) = lines.get_line_span(*snip).unwrap();
        let content = &src[from..to];
        print!("{:4}|", snip);
        print!("{}", content);
        if (from <= underscore.0 && to >= underscore.1) {
            // println!("start: {}, end: {}, from: {}, to: {}", start, end, from, to);
            let spaces = " ".repeat(5 + underscore.0 - from);
            print!("{}", spaces);
            let underscore = "^".repeat(underscore.1 - underscore.0 + 1);
            println!("{}", underscore);
        }
    }
}

fn show_lexer_error(err: lexer::LexicalError) {
    println!("{:#?}", err);
}

fn show_unrecognized_token_error(
    location: &str,
    lines: &Lines,
    src: &str,
    token: &Option<(usize, lexer::Token, usize)>,
    mut expected: Vec<String>,
) {
    remove_extra_quotes(&mut expected);
    match token {
        None => println!("Unexpected end of file. Expected tokens: {:?}", expected),
        Some((start, token, end)) => {
            use lexer::Token::*;

            let message = match token {
                BraceOpen => "unexpected block delimiter: '{'".to_owned(),
                BraceClose => "unexpected block delimiter: '}'".to_owned(),
                x => format!("Expected one of: {:?}, found \"{}\"", expected, token),
            };
            println!("error: {}", message);

            match lines.get_line(*start) {
                None => {
                    println!("in file: {} at unknown location", location);
                }
                Some((line, _, _)) => {
                    println!("--> at: {}:{}", location, line);
                    show_code_snippet(src, line, lines, (*start, *end), None);
                }
            };
        }
    }
}
