#![cfg_attr(
    not(any(feature = "vulkan", feature = "dx12", feature = "gl")),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

extern crate env_logger;
#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;
#[cfg(feature = "gl")]
extern crate gfx_backend_gl as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;
extern crate gfx_hal as hal;

#[cfg(feature = "gl")]
use back::glutin::GlContext;

extern crate glsl_to_spirv;
extern crate image;
extern crate winit;

mod functionality;
pub use functionality::*;
