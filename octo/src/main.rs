use octo::process_file;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Parameters {
    no_test: bool,
    path: Vec<String>,
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap();
    println!("lol");

    let opt = Parameters::from_args();

    println!("Processing files: {:?}", opt);
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

    std::process::exit(if err { 1 } else { 0 });
}
