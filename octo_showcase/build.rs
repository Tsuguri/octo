extern crate octo;

use std::path::PathBuf;
use std::fs;
use std::env;

fn main() {
    let path = "src/file.octo";
    octo::hello(&path);
    //let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    //fs::copy("src/file.octo_bin", dst.join("file.octo_bin"));

    //println!("Desti: {:?}", dst);
}
