use structopt::StructOpt;
use octo::process_file;

use octo::experimental_ir as oei;

#[derive(StructOpt, Debug)]
struct Parameters {
    no_test: bool,
    path: Vec<String>,
}

fn main() {
    stderrlog::new().verbosity(4).init().unwrap();
    println!("lol");


    let opt = Parameters::from_args();

//    if !opt.no_test {
//        let fun = oei::def_user_fun();
//        println!("{:#?}", fun);
//    }

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
