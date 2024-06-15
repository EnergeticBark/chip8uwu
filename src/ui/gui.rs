use egui::Context;
use crate::chip8::State;
use crate::ui;
use crate::ui::Disassembler;

pub struct Gui {
    pub disassembler: Disassembler,
}

impl Gui {
    pub fn new() -> Self {
        Self {
            disassembler: Disassembler::new(),
        }
    }

    pub fn ui(&mut self, ctx: &Context, chip8_state: &mut State) {
        ui::top_bar::draw(ctx, self, chip8_state);
        self.disassembler.draw(ctx, &chip8_state);
    }
}