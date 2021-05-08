use std::fs;
use std::time;

use log::error;
use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;

mod chip8;
mod wrappers;
mod ui;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = wrappers::window::init_window(&event_loop);
    let (mut pixels, mut gui) = wrappers::window::init_pixels_and_gui(&window);
    let mut ui_state = ui::State::new();

    let mut state = chip8::State::new();
    let mut disassembler = ui::Disassembler::new();

    let mut start_time = time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        gui.handle_event(&event);

        if let Event::RedrawRequested(_) = event {
            state.draw(pixels.get_frame());

            gui.prepare();
            gui.ui(|ctx| {
                ui::top_bar::draw(ctx, &mut ui_state, &mut state);
                disassembler.draw(ctx, &mut ui_state, &state);
            });

            let render_result = pixels.render_with(|encoder, render_target, context| {
                context.scaling_renderer.render(encoder, render_target);

                gui.render(encoder, render_target, context);
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
            if input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(scale_factor) = input.scale_factor() {
                gui.scale_factor(scale_factor);
            }

            if let Some(size) = input.window_resized() {
                // don't resize if either value is zero to prevent a panic
                // winit doesn't have Minimize events yet, this should be fixed eventually
                if size.width > 0 && size.height > 0 {
                    pixels.resize_surface(size.width, size.height);
                    gui.resize(size.width, size.height);
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