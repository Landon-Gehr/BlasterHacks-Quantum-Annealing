use std::sync::Arc;

use eframe::egui::{
    self, Align, Color32, FontData, FontDefinitions, FontFamily, Layout, RichText, TextEdit,
};

const FIXED_SYSTEM_SIZE: usize = 32;
const FIXED_LX: f64 = 4.0 * std::f64::consts::PI;
const FIXED_LY: f64 = 4.0 * std::f64::consts::PI;
const FIXED_DOMAIN_DISPLAY: &str = "4π";
const FIXED_K_BITS: usize = 3;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1180.0, 760.0])
            .with_min_inner_size([900.0, 620.0])
            .with_title("BlasterHacks Quantum Annealing UI"),
        ..Default::default()
    };

    eframe::run_native(
        "BlasterHacks Quantum Annealing UI",
        options,
        Box::new(|cc| Ok(Box::new(QuantumUiApp::new(cc)))),
    )
}

struct QuantumUiApp {
    coeff_sin_x: f64,
    coeff_sin_y: f64,
    coeff_cos_x: f64,
    coeff_cos_y: f64,
    coeff_sin_xy: f64,
    coeff_cos_xy: f64,
    g: String,
    status: String,
    preview: PreviewData,
}

#[derive(Default)]
struct PreviewData {
    valid: bool,
    summary: String,
    qubo_dimension: usize,
}

/// Implementation of the main application UI for the quantum annealing preview.
/// 
/// Provides helpers for loading demo input, clearing input fields, generating a
/// discrete QUBO preview, and rendering the continuous problem editor and preview panel.
/// 
/// Uses `egui` to build a form for the forcing function coefficients and boundary
/// condition, then displays a textual representation of the resulting PDE and QUBO
/// summary.
/// 
/// For text rendering, `egui` supports more than `ui.monospace(...)`:
/// - `ui.label(...)` for normal text
/// - `ui.heading(...)`, `ui.small(...)` for different text sizes
/// - `RichText::new(...).strong()` for bold text
/// - `RichText::new(...).color(...)` for colored text
/// - `ui.add(...)` with `TextEdit::singleline(...)` for editable text fields

impl QuantumUiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_math_font(&cc.egui_ctx);

        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.spacing.item_spacing = egui::vec2(10.0, 10.0);
        style.visuals.override_text_color = Some(Color32::from_rgb(235, 239, 244));
        cc.egui_ctx.set_style(style);

        let mut app = Self {
            coeff_sin_x: 1.0,
            coeff_sin_y: 1.0,
            coeff_cos_x: 1.0,
            coeff_cos_y: 1.0,
            coeff_sin_xy: 1.0,
            coeff_cos_xy: 1.0,
            g: "0".to_owned(),
            status: "Edit the system and press Generate Preview.".to_owned(),
            preview: PreviewData::default(),
        };
        app.load_demo_system();
        app
    }

    fn load_demo_system(&mut self) {
        self.coeff_sin_x = 1.0;
        self.coeff_sin_y = 1.0;
        self.coeff_cos_x = 1.0;
        self.coeff_cos_y = 1.0;
        self.coeff_sin_xy = 1.0;
        self.coeff_cos_xy = 1.0;
        self.g = "0".to_owned();
        self.generate_preview();
    }

    fn clear_inputs(&mut self) {
        self.coeff_sin_x = 1.0;
        self.coeff_sin_y = 1.0;
        self.coeff_cos_x = 1.0;
        self.coeff_cos_y = 1.0;
        self.coeff_sin_xy = 1.0;
        self.coeff_cos_xy = 1.0;
        self.g = "0".to_owned();
        self.status = "Cleared inputs.".to_owned();
        self.preview = PreviewData::default();
    }

    fn generate_preview(&mut self) {
        let parsed = match self.parse_form() {
            Ok(parsed) => parsed,
            Err(error) => {
                self.status = error;
                self.preview = PreviewData::default();
                return;
            }
        };

        let qubo_dimension = FIXED_SYSTEM_SIZE * FIXED_K_BITS;

        self.preview = PreviewData {
            valid: true,
            summary: format!(
                "Domain: Ω = (0, {domain}) × (0, {domain}) ≈ (0, {lx:.3}) × (0, {ly:.3})\nDiscrete size n = {n}\nInternal QUBO dimension: {qubo_dimension}",
                domain = FIXED_DOMAIN_DISPLAY,
                lx = parsed.lx,
                ly = parsed.ly,
                n = FIXED_SYSTEM_SIZE,
                qubo_dimension = qubo_dimension,
            ),
            qubo_dimension,
        };
        self.status =
            "Preview generated. The next step is wiring this action into qubo::compute_qubo."
                .to_owned();
    }

    fn parse_form(&self) -> Result<ParsedForm, String> {
        Ok(ParsedForm {
            lx: FIXED_LX,
            ly: FIXED_LY,
        })
    }

    fn continuous_problem_editor(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("Poisson problem").strong().size(18.0));
        ui.add_space(4.0);
        ui.monospace(format!(
            "Domain: Ω = (0, {FIXED_DOMAIN_DISPLAY}) × (0, {FIXED_DOMAIN_DISPLAY})"
        ));

        ui.add_space(8.0);
        ui.label("Forcing function");
        ui.label("Each coefficient is editable in-place and constrained to [-10, 10].");
        ui.add_space(6.0);
        ui.horizontal_wrapped(|ui| {
            ui.label("f(x, y) =");
            coefficient_box(ui, &mut self.coeff_sin_x);
            ui.label("sin(x) +");
            coefficient_box(ui, &mut self.coeff_sin_y);
            ui.label("sin(y) +");
            coefficient_box(ui, &mut self.coeff_cos_x);
            ui.label("cos(x) +");
            coefficient_box(ui, &mut self.coeff_cos_y);
            ui.label("cos(y) +");
            coefficient_box(ui, &mut self.coeff_sin_xy);
            ui.label("sin(x)sin(y) +");
            coefficient_box(ui, &mut self.coeff_cos_xy);
            ui.label("cos(x)cos(y)");
        });

        ui.add_space(8.0);
        ui.label("Boundary data");
        ui.add_space(8.0);
        egui::Grid::new("boundary_grid")
            .num_columns(2)
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
                ui.label("g on ∂Ω");
                ui.add_sized([180.0, 28.0], TextEdit::singleline(&mut self.g));
                ui.end_row();
            });
    }

    fn preview_panel(&self, ui: &mut egui::Ui) {
        ui.label(RichText::new("QUBO preview").strong().size(20.0));
        ui.add_space(6.0);
        ui.label(self.status.as_str());
        ui.add_space(12.0);

        if !self.preview.valid {
            ui.colored_label(
                Color32::from_rgb(255, 190, 110),
                "No preview yet. Populate the form and generate one.",
            );
            return;
        }

        ui.group(|ui| {
            ui.label(RichText::new("Continuous model").strong());
            ui.monospace(format!(
                "Domain:\nΩ = (0, {}) × (0, {})\n\nPDE:\n-Δu(x, y) = {} in Ω\n\nBoundary condition:\nu(x, y) = {} on ∂Ω",
                FIXED_DOMAIN_DISPLAY,
                FIXED_DOMAIN_DISPLAY,
                self.forcing_expression(),
                self.g
            ));
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label(RichText::new("Summary").strong());
            ui.monospace(self.preview.summary.as_str());
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label(RichText::new("Integration notes").strong());
            ui.label(format!(
                "Your solver will produce a {}x{} QUBO matrix from this configuration.",
                self.preview.qubo_dimension, self.preview.qubo_dimension
            ));
            ui.label("The preview is using dense helper math so the UI stays independent for now.");
            ui.label("When you are ready, replace Generate Preview with a call into the `qubo` crate.");
        });
    }

    fn forcing_expression(&self) -> String {
        format!(
            "{:.1} sin(x) + {:.1} sin(y) + {:.1} cos(x) + {:.1} cos(y) + {:.1} sin(x)sin(y) + {:.1} cos(x)cos(y)",
            self.coeff_sin_x,
            self.coeff_sin_y,
            self.coeff_cos_x,
            self.coeff_cos_y,
            self.coeff_sin_xy,
            self.coeff_cos_xy,
        )
    }
}

fn install_math_font(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "stix_two_math".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/STIXTwoMath.otf"
        ))),
    );

    if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
        family.push("stix_two_math".to_owned());
    }
    if let Some(family) = fonts.families.get_mut(&FontFamily::Monospace) {
        family.push("stix_two_math".to_owned());
    }

    ctx.set_fonts(fonts);
}

fn coefficient_box(ui: &mut egui::Ui, value: &mut f64) {
    ui.add_sized(
        [26.0, 12.0],
        egui::DragValue::new(value)
            .range(-10.0..=10.0)
            .speed(0.1),
    );
}

impl eframe::App for QuantumUiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar")
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(17, 22, 31))
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ctx, |ui| {
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    ui.heading(
                        RichText::new("Quantum Annealing Workbench")
                            .size(28.0)
                            .color(Color32::from_rgb(121, 200, 255)),
                    );
                    ui.separator();
                    ui.label("Choose the Poisson forcing coefficients and boundary function.");
                });
            });

        egui::SidePanel::left("controls")
            .resizable(false)
            .default_width(300.0)
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(22, 28, 39))
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ctx, |ui| {
                ui.label(RichText::new("Controls").strong().size(22.0));
                ui.add_space(8.0);
                ui.label(format!("System size n is fixed at {}", FIXED_SYSTEM_SIZE));

                ui.add_space(16.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Generate Preview").clicked() {
                        self.generate_preview();
                    }
                    if ui.button("Load Demo").clicked() {
                        self.load_demo_system();
                    }
                    if ui.button("Clear").clicked() {
                        self.clear_inputs();
                    }
                });

                ui.add_space(12.0);
                ui.group(|ui| {
                    ui.label(RichText::new("What this starter includes").strong());
                    ui.label("Fixed Poisson PDE on a fixed domain Ω = (0, 4π) × (0, 4π)");
                    ui.label("An inline trig forcing function editor with coefficients in [-10, 10]");
                    ui.label("A fixed discrete size n = 32");
                    ui.label("One boundary function g on ∂Ω");
                    ui.label("Hidden internal QUBO settings");
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .fill(Color32::from_rgb(10, 14, 20))
                    .inner_margin(egui::Margin::same(18)),
            )
            .show(ctx, |ui| {
                ui.columns(2, |columns| {
                    egui::ScrollArea::vertical()
                        .id_salt("editors_scroll")
                        .show(&mut columns[0], |ui| {
                            self.continuous_problem_editor(ui);
                        });

                    egui::ScrollArea::vertical()
                        .id_salt("preview_scroll")
                        .show(&mut columns[1], |ui| {
                            self.preview_panel(ui);
                        });
                });
            });
    }
}

struct ParsedForm {
    lx: f64,
    ly: f64,
}
