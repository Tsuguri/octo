#[macro_use]
extern crate structopt;

extern crate lalrpop_util;
extern crate octo_parser;

mod semantics;

use std::fs::File;
use std::io::prelude::*;
use std::string::ToString;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Parameters {
    path: Option<String>,
    lex: bool,
}

fn run_from_file(filepath: String, lex: bool) -> bool {
    println!("Running from file: {}", filepath);
    match File::open(&filepath) {
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
                    return interpret(&filepath, &contents, lex);
                }
            }
        }
    }
}

fn main() {
    let opt = Parameters::from_args();
    match opt.path {
        Some(path) => ::std::process::exit(if run_from_file(path, opt.lex) { 0 } else { 1 }),
        None => {
            //run_interpreter(opt.lex);
            ::std::process::exit(1);
        }
    }
}

fn interpret(location: &str, data: &str, lex: bool) -> bool {
    match octo_parser::parse(location, data, lex) {
        Ok(something) => match semantics::analyze(something) {
            Result::Ok(_) => true,
            Result::Err(errors) => {
                for error in errors {
                    println!("--> {:?}", error);
                }
                false
            }
        },
        Err(warning) => {
            println!("Command failed: {:?}", warning);
            false
        }
    }
}
