use std::time;

use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::ui::framework::Framework;

mod chip8;
mod ui;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(900, 480);
        WindowBuilder::new()
            .with_title("chip8uwu")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut framework = Framework::new(&event_loop, &window, WIDTH, HEIGHT);

    let mut state = chip8::State::new();
    let mut delay_timer = time::Instant::now();
    let mut clock_timer = time::Instant::now();

    event_loop
        .run(|event, elwt| {
            if input.update(&event) {
                if input.close_requested() {
                    elwt.exit();
                    return;
                }

                if let Some(scale_factor) = input.scale_factor() {
                    framework.scale_factor(scale_factor);
                }

                if let Some(size) = input.window_resized() {
                    framework.resize(size.width, size.height);
                }

                let delay_timer_delta = time::Instant::now() - delay_timer;
                if delay_timer_delta > time::Duration::from_millis(16) && state.rom_loaded {
                    state.delay = state.delay.saturating_sub(1); // Decrement the delay register.
                    delay_timer = time::Instant::now();
                }
                state.keyboard = [
                    input.key_held(KeyCode::KeyX),
                    input.key_held(KeyCode::Digit1),
                    input.key_held(KeyCode::Digit2),
                    input.key_held(KeyCode::Digit3),
                    input.key_held(KeyCode::KeyQ),
                    input.key_held(KeyCode::KeyW),
                    input.key_held(KeyCode::KeyE),
                    input.key_held(KeyCode::KeyA),
                    input.key_held(KeyCode::KeyS),
                    input.key_held(KeyCode::KeyD),
                    input.key_held(KeyCode::KeyZ),
                    input.key_held(KeyCode::KeyC),
                    input.key_held(KeyCode::Digit4),
                    input.key_held(KeyCode::KeyR),
                    input.key_held(KeyCode::KeyF),
                    input.key_held(KeyCode::KeyV),
                ];
                let clock_timer_delta = time::Instant::now() - clock_timer;
                if state.rom_loaded {
                    for _ in 0..clock_timer_delta.as_millis() / 2 {
                        state.emulate().unwrap(); // Execute the next instruction.
                    }
                    clock_timer = time::Instant::now();
                }

                window.request_redraw();
            }

            match event {
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    framework.screen.draw(&framework.queue, state.frame_rgba());

                    framework.prepare(&window, &mut state);

                    framework.render();
                }
                Event::WindowEvent { event, .. } => {
                    framework.handle_event(&window, &event);
                }
                _ => (),
            }
        })
        .unwrap();
}
