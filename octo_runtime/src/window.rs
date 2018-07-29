use backend as back;
use hal;
use hal::Instance;
use std;
use winit;

#[cfg(any(feature = "vulkan", feature = "dx12"))]
pub type WindowType = winit::Window;
#[cfg(feature = "gl")]
pub type WindowType = i32;

#[cfg(not(feature = "gl"))]
pub fn build_window(
    name: &str,
    wb: winit::WindowBuilder,
    events_loop: &mut winit::EventsLoop,
) -> (
    WindowType,
    back::Instance,
    std::vec::Vec<hal::Adapter<back::Backend>>,
    <back::Backend as hal::Backend>::Surface,
) {
    let window = wb.build(&events_loop).unwrap();
    let instance = back::Instance::create(name, 1);
    let surface = instance.create_surface(&window);
    let adapters = instance.enumerate_adapters();
    (window, instance, adapters, surface)
}

#[cfg(feature = "gl")]
pub fn build_window(
    _name: &str,
    wb: winit::WindowBuilder,
    events_loop: &mut winit::EventsLoop,
) -> (
    WindowType,
    i32, // totally never used
    std::vec::Vec<hal::Adapter<back::Backend>>,
    <back::Backend as hal::Backend>::Surface,
) {
    let window = {
        let builder =
            back::config_context(back::glutin::ContextBuilder::new(), ColorFormat::SELF, None)
                .with_vsync(true);
        back::glutin::GlWindow::new(wb, builder, &events_loop).unwrap()
    };

    let surface = back::Surface::from_window(window);
    let adapters = surface.enumerate_adapters();
    (0i32, 0i32, adapters, surface)
}

pub fn window_builder(name: String, width: f64, height: f64) -> winit::WindowBuilder {
    winit::WindowBuilder::new()
        .with_dimensions(winit::dpi::LogicalSize::from_physical(
            winit::dpi::PhysicalSize {
                width: width,
                height: height,
            },
            1.0,
        ))
        .with_title(name)
}
