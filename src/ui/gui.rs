use std::fs;

use egui::load::SizedTexture;
use egui::Context;

use crate::chip8::State;
use crate::ui;
use crate::ui::{Disassembler, Registers};

pub struct Gui {
    pub disassembler: Disassembler,
    pub registers: Registers,
    texture: SizedTexture, // Chip8's Pixels framebuffer as a texture.
}

impl Gui {
    pub fn new(texture: SizedTexture) -> Self {
        Self {
            disassembler: Disassembler::new(),
            registers: Registers::new(),
            texture,
        }
    }

    pub fn ui(&mut self, ctx: &Context, chip8_state: &mut State) {
        ui::top_bar::draw(ctx, self, chip8_state);
        self.disassembler.draw(ctx, chip8_state);
        self.registers.draw(ctx, chip8_state);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(
                egui::Image::new(self.texture)
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
