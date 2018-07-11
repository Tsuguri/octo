#[macro_use]
extern crate structopt;

extern crate lalrpop_util;
extern crate octo_parser;

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::string::ToString;
use structopt::StructOpt;

use octo_parser::ast;
use octo_parser::grammar::ProgramParser;

#[derive(StructOpt, Debug)]
struct Parameters {
    path: Option<String>,
}

fn run_interpreter() {
    println!("Running interpreter",);
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut command_buffer = String::new();
        io::stdin().read_line(&mut command_buffer).unwrap();
        println!("Got command: {}", command_buffer.trim_right());
        interpret(command_buffer.trim_right());
    }
}

fn run_from_file(filepath: String) -> bool {
    println!("Running from file: {}", filepath);
    match File::open(filepath) {
        Result::Err(_) => {
            println!("File nout found!");
            return false;
        }
        Result::Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Result::Err(err) => {
                    println!("Error reading file: {}", err);
                    return false;
                }
                Result::Ok(_) => {
                    return interpret(&contents);
                }
            }
        }
    }
}

fn main() {
    let opt = Parameters::from_args();
    match opt.path {
        Some(path) => ::std::process::exit(if run_from_file(path) { 0 } else { 1 }),
        None => {
            run_interpreter();
            ::std::process::exit(0);
        }
    }
}

fn interpret(data: &str) -> bool {
    match parse(data) {
        Ok(something) => true,
        Err(warning) => {
            println!("Command failed: {}", warning);
            false
        }
    }
}

fn parse(
    data: &str,
) -> Result<
    octo_parser::ast::Program,
    lalrpop_util::ParseError<usize, octo_parser::lexer::Token, octo_parser::lexer::LexicalError>,
> {
    let result = ProgramParser::new().parse(octo_parser::lexer::Lexer::new(data));
    println!("{:#?}", result);
    result
}
