
use std::path::PathBuf;
use std::fs;
use std::env;

fn main() {
    let path = "src/file.octo";
    octo::process_file(&path);
}
