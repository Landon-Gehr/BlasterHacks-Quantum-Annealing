use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 500.0])
            .with_title("x² plot"),
        ..Default::default()
    };

    eframe::run_native(
        "x² plot",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    input: String,
    submitted: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("y = x²");

            let points: PlotPoints = (-100..=100)
                .map(|i| {
                    let x = i as f64 * 0.1;
                    [x, x * x]
                })
                .collect();

            Plot::new("x_squared")
                .view_aspect(2.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(Line::new(points).name("x²"));
                });

            ui.separator();

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.input);
                if ui.button("Submit").clicked() {
                    self.submitted = self.input.clone();
                }
            });

            if !self.submitted.is_empty() {
                ui.label(format!("You entered: {}", self.submitted));
            }
        });
    }
}