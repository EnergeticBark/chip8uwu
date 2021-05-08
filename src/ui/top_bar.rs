use crate::ui::state::State;

pub fn draw(ctx: &egui::CtxRef, state: &mut State) {
    egui::TopPanel::top("menubar_container").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Open...").clicked() {
                    state.disassembler_open = true;
                }
            });
            egui::menu::menu(ui, "Tools", |ui| {
                if ui.button("Disassemble...").clicked() {
                    state.disassembler_open = true;
                }
            });
        });
    });
}