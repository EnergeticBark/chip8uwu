use egui::{Color32, FontId};
use egui::TextStyle::Body;

use crate::chip8;
use crate::ui;

pub struct Disassembler {
    follow_current: bool,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            follow_current: true,
        }
    }

    fn draw_line(&self, ui: &mut egui::Ui, list_pc: usize, chip8_pc: u16, bytes: &[u8]) {
        let (instr, args) = {
            if let Ok(op) = chip8::Op::new(bytes[0], bytes[1]) {
                op.disassemble()
            } else {
                (String::new(), String::new())
            }
        };
        ui.horizontal_wrapped(|ui| {
            // if both are zeros, draw grayed out text
            if bytes[0] == 0 && bytes[1] == 0 {
                ui.visuals_mut().override_text_color = Some(Color32::from_rgb(100, 100, 100));
            } else if list_pc as u16 == chip8_pc {
                ui.visuals_mut().override_text_color = Some(Color32::LIGHT_GRAY);
            }

            ui.label(format!(
                "{:04x}: {:02x}{:02x} ",
                list_pc, bytes[0], bytes[1]
            ));
            ui.colored_label(Color32::from_rgb(128, 140, 255), instr);
            ui.label(args);
        });
    }

    fn draw_list(&self, ui: &mut egui::Ui, chip8_state: &chip8::State) {
        ui.style_mut().text_styles.insert(Body, FontId::monospace(11.0));
        let row_height = ui.text_style_height(&Body);
        ui.spacing_mut().interact_size.y = row_height;
        ui.spacing_mut().item_spacing.y = 0.0;
        let instructions = chip8_state.memory[0x200..].chunks_exact(2);
        egui::ScrollArea::vertical()
            .show_rows(
                ui,
                row_height,
                instructions.len(),
                |ui, row_range| {
                    for (i, bytes) in instructions.skip(row_range.start).enumerate() {
                        let list_pc = 0x200 + (row_range.start + i) * 2;
                        self.draw_line(ui, list_pc, chip8_state.pc, bytes);
                        if i > row_range.end {
                            break;
                        }
                    }
                    ui.allocate_space(ui.available_size());
                }
            );
    }

    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        ui_state: &mut ui::State,
        chip8_state: &chip8::State,
    ) {
        egui::Window::new("Disassembler")
            .open(&mut ui_state.disassembler_open)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.label("All instructions starting starting at 0x200");
                ui.separator();
                self.draw_list(ui, chip8_state);
            });
    }
}
