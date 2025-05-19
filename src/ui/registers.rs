use egui::TextStyle::Body;
use egui::{FontId, Ui};
use egui_extras::{Column, TableBody, TableBuilder, TableRow};

use crate::chip8::State;

pub struct Registers {
    pub open: bool,
}

impl Registers {
    pub fn new() -> Self {
        Self { open: false }
    }

    pub fn draw(&mut self, ctx: &egui::Context, chip8_state: &State) {
        egui::SidePanel::left("Registers")
            .resizable(false)
            .show(ctx, |ui| draw_table(ui, chip8_state));
    }
}

fn draw_table(ui: &mut Ui, chip8_state: &State) {
    ui.vertical(|ui| {
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .header(20.0, draw_header_row)
            .body(|body| draw_body_rows(body, chip8_state));
    });
}

fn draw_header_row(mut header: TableRow) {
    header.col(|ui| {
        ui.strong("Register");
    });
    header.col(|ui| {
        ui.strong("Value");
    });
}

fn draw_body_rows(mut body: TableBody, chip8_state: &State) {
    body.ui_mut()
        .style_mut()
        .text_styles
        .insert(Body, FontId::monospace(11.0));
    for v in 0..chip8_state.v.len() {
        body.row(14.0, |mut row| {
            row.col(|ui| {
                ui.label(format!("V{v:01X}"));
            });
            row.col(|ui| {
                ui.label(format!("0x{:02X}", chip8_state.v[v]));
            });
        });
    }
}
