use std::fs;

use native_dialog::FileDialog;

use crate::chip8;
use crate::ui::state::State;

pub fn draw(ctx: &egui::CtxRef, ui_state: &mut State, chip8_state: &mut chip8::State) {
    egui::TopPanel::top("menubar_container").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Open...").clicked() {
                    let path = FileDialog::new()
                        .add_filter("CHIP-8 ROM", &["ch8"])
                        .show_open_single_file()
                        .unwrap();

                    if let Some(path) = path {
                        let rom = fs::read(path).expect("failed to read file");
                        chip8_state.load_rom(&rom);
                    }
                }
            });

            egui::menu::menu(ui, "Tools", |ui| {
                if ui.button("Disassemble...").clicked() {
                    ui_state.disassembler_open = true;
                }
            });
        });
    });
}
