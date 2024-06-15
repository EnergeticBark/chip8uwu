use std::time;

use log::error;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

mod chip8;
mod ui;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = ui::window::init_window(&event_loop);
    let (mut pixels, mut framework) = ui::window::init_pixels_and_framework(&window, &event_loop);
    let mut state = chip8::State::new();
    let mut start_time = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent { ref event, .. } = event {
            framework.handle_event(&event);
        }

        if let Event::RedrawRequested(_) = event {
            state.draw(pixels.frame_mut());

            framework.prepare(&window, &mut state);

            let render_result = pixels.render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);

                framework.render(encoder, render_target, context);

                Ok(())
            });

            if render_result
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            if let Some(size) = input.window_resized() {
                // don't resize if either value is zero to prevent a panic
                // winit doesn't have Minimize events yet, this should be fixed eventually
                if size.width > 0 && size.height > 0 {
                    pixels.resize_surface(size.width, size.height).unwrap();
                    framework.resize(size.width, size.height);
                }
            }

            let delta_time = time::Instant::now() - start_time;
            if delta_time > time::Duration::from_millis(16) && state.rom_loaded {
                state.emulate().unwrap();
                start_time = time::Instant::now();
            }
            window.request_redraw();
        }
    });
}