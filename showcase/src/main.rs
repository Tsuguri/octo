use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        // .filter_module("just_renderer", LevelFilter::Trace)
        // .filter_module("just_game", LevelFilter::Trace)
        .init();

    just_game::run();
}
