use std::time;

use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::ui::framework::Framework;

mod chip8;
mod ui;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(640, 480);
        WindowBuilder::new()
            .with_title("chip8uwu")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };


    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };
    let mut state = chip8::State::new();
    let mut start_time = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height).unwrap();
                framework.resize(size.width, size.height);
            }

            let delta_time = time::Instant::now() - start_time;
            if delta_time > time::Duration::from_millis(16) && state.rom_loaded {
                state.emulate().unwrap();
                start_time = time::Instant::now();
            }
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                framework.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                state.draw(pixels.frame_mut());

                framework.prepare(&window, &mut state);

                let render_result = pixels.render_with(|encoder, render_target, context| {
                    context.scaling_renderer.render(encoder, render_target);

                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                if let Err(err) = render_result {
                    error!("pixels.render() failed: {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}