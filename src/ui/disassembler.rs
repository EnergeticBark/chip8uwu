use egui::Color32;

use crate::chip8;
use crate::ui;

const LIST_HEIGHT: f32 = 200.0;

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
        ui.style_mut().body_text_style = egui::TextStyle::Monospace;
        let line_height = ui
            .fonts()
            .row_height(egui::epaint::text::TextStyle::Monospace);
        ui.spacing_mut().interact_size.y = line_height;
        ui.spacing_mut().item_spacing.y = 0.0;

        let instructions = chip8_state.memory[0x200..].chunks_exact(2);
        let total_height_px = instructions.len() as f32 * line_height;
        let current_scroll = {
            let margin = ui.visuals().clip_rect_margin;
            ui.clip_rect().top() - ui.min_rect().top() + margin
        };

        // start drawing the list, and do some tricks to only draw the lines we need
        let skip_first = (current_scroll / line_height).floor() as usize;
        let top_padding = skip_first as f32 * line_height;
        // draw an empty space above where we're currently scrolled
        ui.add_space(top_padding);

        let lines_to_draw = (LIST_HEIGHT / line_height).ceil() as usize;
        for (i, bytes) in instructions.skip(skip_first).enumerate() {
            let list_pc = 0x200 + (skip_first + i) * 2;
            self.draw_line(ui, list_pc, chip8_state.pc, bytes);
            if i > lines_to_draw {
                break;
            }
        }

        let remaining_px = total_height_px - top_padding - lines_to_draw as f32 * line_height;
        // draw an empty space below us, however many pixels we have remaining
        ui.add_space(remaining_px);
    }

    pub fn draw(
        &mut self,
        ctx: &egui::CtxRef,
        ui_state: &mut ui::State,
        chip8_state: &chip8::State,
    ) {
        egui::Window::new("Disassembler")
            .open(&mut ui_state.disassembler_open)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("All instructions starting starting at 0x200");
                ui.separator();
                egui::ScrollArea::from_max_height(LIST_HEIGHT).show(ui, |ui| {
                    ui.vertical(|ui| {
                        self.draw_list(ui, chip8_state);
                    });
                });
            });
    }
}
