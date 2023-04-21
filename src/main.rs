use eframe::{egui::{self, InputState, RawInput, Sense, Event}, epaint::{Rect, ClippedShape}};
use sub::SubGui;
mod sub;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );
}

struct MyEguiApp {
    sub: SubGui,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { sub: SubGui::new() }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Boring host heading...");

            let (rect, response) = ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

            let mut full_output = ui.ctx().input(|input_state| {
                let raw_input = convert_subwindow_input(input_state, rect);
                self.sub.run(raw_input)
            });

            for ClippedShape(_, shape) in &mut full_output.shapes {
                shape.translate(rect.left_top().to_vec2());
                ui.painter().add(shape.clone());
            }
        });
    }
}

fn convert_subwindow_input(input_state: &InputState, rect: Rect) -> RawInput {
    let mut raw = input_state.raw.clone();
    for ev in &mut raw.events {
        match ev {
            Event::PointerMoved(new_pos) => {
                *new_pos -= rect.left_top().to_vec2();
            },
            Event::PointerButton { pos, .. } => {
                *pos -= rect.left_top().to_vec2();
            }
            _ => (),
        }
    }

    raw
}