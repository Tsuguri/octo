extern crate env_logger;
extern crate octo_runtime;
use octo_runtime::main_function;
use octo_runtime::winit;

fn main() {
    env_logger::init();
    //let mut events_loop = winit::EventsLoop::new();
    main_function();//&mut events_loop);
}
