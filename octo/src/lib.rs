extern crate lalrpop_util;

use std::path::Path;
use std::io::{Write, Read};
use std::fs::File;
use octo_runtime::OctoModule;

pub fn process_file(path: &str) {
    println!("Processing file at: {}", path);
    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");

    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    // do something here;

    let mut ast = match parser::parse(path, &data, false) {
        Err(warning) => {
            panic!("Command failed: {:?}", warning);
        },
        Ok(ast) => ast,
    };

    semantics::analyze(ast).map_err(|errs| {
        for error in errs {
            println!("--> {:?}", error);
        }
    }).unwrap();

    let module = OctoModule::new();
    
    let module_data = serde_json::to_string(&module).unwrap();

    let mut output_file = File::create(&result_path).unwrap();
    output_file.write_all(module_data.as_bytes()).unwrap();

    println!("processed as: {:?}", result_path);

}
pub mod semantics;
