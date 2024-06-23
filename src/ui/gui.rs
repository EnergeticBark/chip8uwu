use egui::Context;

use crate::chip8::State;
use crate::ui;
use crate::ui::{Disassembler, Registers};

pub struct Gui {
    pub disassembler: Disassembler,
    pub registers: Registers,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            disassembler: Disassembler::new(),
            registers: Registers::new(),
        }
    }

    pub fn ui(&mut self, ctx: &Context, chip8_state: &mut State) {
        ui::top_bar::draw(ctx, self, chip8_state);
        self.disassembler.draw(ctx, chip8_state);
        self.registers.draw(ctx, chip8_state);
    }
}
