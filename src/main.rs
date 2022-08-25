use eframe::egui;
mod fronius;
mod utils;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(egui::vec2(250.0,150.0));
    eframe::run_native(
        "egui app",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    powers: fronius::Power
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            powers: fronius::Power { p_grid: 0.0, p_load: 0.0, p_pv: 0.0 }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rust egui");
            if ui.button("Update").clicked() {
                let content = fronius::get();
                self.powers = fronius::retrieve_power_measure(&content);
            }
            ui.label(format!("Solar power : {}", utils::format_generic_power(self.powers.p_pv)));
            ui.label(format!("Load power : {}", utils::format_generic_power(self.powers.p_load)));
            ui.label(format!("Balance to grid : {}", utils::format_grid_power(self.powers.p_grid)));
        });
    }
}

