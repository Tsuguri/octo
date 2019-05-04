#[macro_use]
extern crate structopt;

use structopt::StructOpt;
use octo::process_file;

#[derive(StructOpt, Debug)]
struct Parameters {
    path: Vec<String>,
}

fn main() {
    let opt = Parameters::from_args();

    for file in opt.path {
        process_file(&file);
    }
}
