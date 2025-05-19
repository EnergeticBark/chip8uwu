use std::{thread, time};

use eframe::Frame;
use egui::{Context, Key, ViewportBuilder};

mod chip8;
mod ui;

struct App {
    chip8: chip8::State,
    gui: ui::gui::Gui,
    delay_timer: time::Instant,
}

impl App {
    fn new() -> Self {
        App {
            chip8: chip8::State::new(),
            gui: ui::gui::Gui::new(),
            delay_timer: time::Instant::now(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        ctx.input(|i| {
            let keys = [
                Key::X,
                Key::Num1,
                Key::Num2,
                Key::Num3,
                Key::Q,
                Key::W,
                Key::E,
                Key::A,
                Key::S,
                Key::D,
                Key::Z,
                Key::C,
                Key::Num4,
                Key::R,
                Key::F,
                Key::V,
            ];
            
            if self.delay_timer.elapsed() > time::Duration::from_millis(16) {
                self.chip8.delay = self.chip8.delay.saturating_sub(1); // Decrement the delay register.
                self.delay_timer = time::Instant::now();
            }
            self.chip8.keyboard = keys.map(|key| i.key_down(key));

            let cycles_to_run = (500.0 * i.unstable_dt) as usize;

            if self.chip8.rom_loaded {
                for _ in 0..cycles_to_run {
                    self.chip8.emulate().unwrap();
                }
            }
        });

        self.gui.ui(ctx, &mut self.chip8);
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_min_inner_size([900.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "chip8uwu",
        native_options,
        Box::new(|cc| {
            let egui_ctx = cc.egui_ctx.clone();
            thread::spawn(move || loop {
                thread::sleep(time::Duration::from_millis(1));
                egui_ctx.request_repaint();
            });
            Ok(Box::new(App::new()))
        }),
    )
    .unwrap();
}
