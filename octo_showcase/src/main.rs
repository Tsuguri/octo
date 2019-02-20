//extern crate simple_logger;
//extern crate octo_runtime;
//extern crate log;
use octo_runtime::main_function;
//use octo_runtime::winit;
#[allow(unused_imports)]
use log::{error, warn, info, debug, trace};

fn main() {
    //env_logger::init();
    
    //let mut events_loop = winit::EventsLoop::new();
    
    simple_logger::init().unwrap();

    info!("whoop whoop");

    main_function();//&mut events_loop);
}
