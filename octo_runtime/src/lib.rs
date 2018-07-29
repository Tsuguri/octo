#![cfg_attr(
    not(any(feature = "vulkan", feature = "dx12", feature = "gl")),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

#[cfg(feature = "dx12")]
pub extern crate gfx_backend_dx12 as backend;
#[cfg(feature = "gl")]
pub extern crate gfx_backend_gl as backend;
#[cfg(feature = "vulkan")]
pub extern crate gfx_backend_vulkan as backend;
pub extern crate gfx_hal as hal;

#[cfg(feature = "gl")]
use back::glutin::GlContext;

extern crate glsl_to_spirv;
pub extern crate image;
pub extern crate winit;

mod data_loading;
mod functionality;
mod types;
mod window;
pub use functionality::*;
