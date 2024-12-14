use just_game::run;
use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Error)
        // .filter_module("just_renderer", LevelFilter::Trace)
        // .filter_module("just_game", LevelFilter::Trace)
        .init();

    pollster::block_on(run());
}
