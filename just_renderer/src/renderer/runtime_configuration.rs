pub struct RuntimeConfiguration {
    pub render_scene: bool,
    pub render_gui: bool,
}

impl Default for RuntimeConfiguration {
    fn default() -> Self {
        RuntimeConfiguration {
            render_gui: true,
            render_scene: true,
        }
    }
}
