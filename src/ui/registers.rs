use egui_extras::{Column, TableBuilder};
use crate::chip8;

pub struct Registers {
    pub open: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            open: false,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &egui::Context,
        chip8_state: &chip8::State,
    ) {
        egui::Window::new("Registers")
            .open(&mut self.open)
            .auto_sized()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(Column::auto())
                            .column(Column::auto())
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.strong("Register");
                                });
                                header.col(|ui| {
                                    ui.strong("Value");
                                });
                            })
                            .body(|mut body| {
                                for v in 0..chip8_state.v.len() {
                                    body.row(14.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(format!("V{:01X}", v));
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("0x{:02X}", chip8_state.v[v]));
                                        });
                                    });
                                }
                            });
                    });
                });
            });
    }
}