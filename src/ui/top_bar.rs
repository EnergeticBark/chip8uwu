use std::fs;

use native_dialog::DialogBuilder;

use crate::chip8;
use crate::ui::gui::Gui;

pub fn draw(ctx: &egui::Context, ui_state: &mut Gui, chip8_state: &mut chip8::State) {
    egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "File", |ui| {
                if ui.button("Open...").clicked() {
                    let path = DialogBuilder::file()
                        .add_filter("CHIP-8 ROM", ["ch8"])
                        .open_single_file()
                        .show()
                        .unwrap();

                    if let Some(path) = path {
                        let rom = fs::read(path).expect("failed to read file");
                        chip8_state.load_rom(&rom);
                    }
                    ui.close_menu();
                }
            });

            egui::menu::menu_button(ui, "Tools", |ui| {
                if ui.button("Disassemble...").clicked() {
                    ui_state.disassembler.open = true;
                    ui.close_menu();
                }
                if ui.button("Registers...").clicked() {
                    ui_state.registers.open = true;
                    ui.close_menu();
                }
            });
        });
    });
}
