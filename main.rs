use eframe::egui;
mod roster_mgmt;

fn main() {
    roster_mgmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Volunteerer",
        native_options,
        Box::new(|cc| Ok(Box::new(RustVolunteererApp::new(cc)))),
    );
}

#[derive(Default)]
struct RustVolunteererApp {}

impl RustVolunteererApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for RustVolunteererApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(4.0);
        let (volunteer, cw) = roster_mgmt::get_volunteer_for_current_week();

        egui::Area::new(egui::Id::new("main_content"))
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
            // .fixed_pos(egui::pos2(32.0, 32.0))
            .show(ctx, |ui| {
                ui.label(format!("Current week: {cw}"));
                ui.separator();
                ui.label(format!("Our volunteer: {volunteer}"));
            });

        // egui::CentralPanel::default().show(ctx, |_ui| {
        // });

        // ui.columns(3, |columns| {
        //     columns[0].vertical_centered(|_ui| {});
        //     columns[1].vertical_centered(|mid_col| {
        //         mid_col.horizontal_centered(|ui| {
        //         });
        //     });
        //     columns[2].vertical_centered(|_ui| {});
        // });
    }
}
