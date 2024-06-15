use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use pixels::{Pixels, SurfaceTexture};
use crate::ui::framework::Framework;

pub fn init_window<T>(event_loop: &EventLoop<T>) -> Window {
    let size = LogicalSize::new(640, 480);
    WindowBuilder::new()
        .with_title("chip8uwu")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

pub fn init_pixels_and_framework<T>(
    window: &Window,
    event_loop: &EventLoop<T>
) -> (Pixels, Framework) {
    let window_size = window.inner_size();
    let scale_factor = window.scale_factor() as f32;
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
    let pixels = Pixels::new(crate::WIDTH, crate::HEIGHT, surface_texture).unwrap();
    let framework = Framework::new(
        &event_loop,
        window_size.width,
        window_size.height,
        scale_factor,
        &pixels,
    );

    (pixels, framework)
}