use structopt::StructOpt;
use octo::process_file;

#[derive(StructOpt, Debug)]
struct Parameters {
    path: Vec<String>,
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap();

    let opt = Parameters::from_args();

    let mut err = false;
    for file in opt.path {
        match process_file(&file) {
            Result::Ok(()) => {}
            Result::Err(()) => {
                log::error!("Compilation of {} failed", file);
                err = true;
            }
        }
    }

    std::process::exit(if err {1} else {0});
}
