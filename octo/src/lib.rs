extern crate lalrpop_util;

use std::path::Path;
use std::io::Write;
use std::fs::File;
use octo_runtime::OctoModule;

pub fn hello(path: &str) {
    println!("Processing file at: {}", path);
    process_file(path);
}

fn process_file(path: &str) {
    let p = Path::new(path);

    if !p.is_file() {
        panic!("given path is not a file: {}", path);
    }
    let result_path = p.with_extension("octo_bin");

    let module = OctoModule::new();
    
    let module_data = serde_json::to_string(&module).unwrap();

    let mut output_file = File::create(&result_path).unwrap();
    output_file.write_all(module_data.as_bytes()).unwrap();

    println!("processed as: {:?}", result_path);

}
pub mod semantics;
