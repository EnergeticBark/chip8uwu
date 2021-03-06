use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use pixels::{Pixels, SurfaceTexture};
use crate::wrappers::gui::Gui;

pub fn init_window<T>(event_loop: &EventLoop<T>) -> Window {
    let size = LogicalSize::new(crate::WIDTH as f64, crate::HEIGHT as f64);
    WindowBuilder::new()
        .with_title("chip8uwu")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

pub fn init_pixels_and_gui(window: &Window) -> (Pixels, Gui) {
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    let pixels = Pixels::new(crate::WIDTH, crate::HEIGHT, surface_texture).unwrap();
    let gui = Gui::new(
        window_size.width,
        window_size.height,
        scale_factor,
        pixels.context()
    );

    (pixels, gui)
}