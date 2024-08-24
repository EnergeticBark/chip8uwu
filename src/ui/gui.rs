use std::fs;

use eframe::epaint::textures::TextureOptions;
use egui::{ColorImage, Context, TextureHandle};

use crate::chip8::State;
use crate::ui;
use crate::ui::{Disassembler, Registers};

pub struct Gui {
    pub disassembler: Disassembler,
    pub registers: Registers,
    screen: Option<TextureHandle>, // Chip8's framebuffer as a texture.
}

impl Gui {
    pub fn new() -> Self {
        Self {
            disassembler: Disassembler::new(),
            registers: Registers::new(),
            screen: None,
        }
    }

    pub fn ui(&mut self, ctx: &Context, chip8_state: &mut State) {
        ui::top_bar::draw(ctx, self, chip8_state);
        self.disassembler.draw(ctx, chip8_state);
        self.registers.draw(ctx, chip8_state);
        egui::CentralPanel::default().show(ctx, |ui| {
            let frame = self.screen.get_or_insert_with(|| {
                ctx.load_texture(
                    "Chip8 Screen",
                    ColorImage::from_gray([64, 32], &[0; 2048]),
                    TextureOptions::NEAREST,
                )
            });

            frame.set(
                ColorImage::from_gray([64, 32], &chip8_state.frame_grayscale()),
                TextureOptions::NEAREST,
            );
            ui.add(
                egui::Image::new(&*frame)
                    .maintain_aspect_ratio(true)
                    .shrink_to_fit(),
            );
        });

        // Loads a rom if it's dragged and dropped onto the window.
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let dropped_file = i.raw.dropped_files.first().unwrap().clone();
                let path = dropped_file.path.unwrap();
                let rom = fs::read(path).expect("failed to read file");
                chip8_state.load_rom(&rom);
            }
        });
    }
}
