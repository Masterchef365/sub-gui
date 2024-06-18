use std::{io::Cursor, time::Instant};

use eframe::{
    egui::{self, Event, InputState, RawInput, Sense},
    epaint::{ClippedShape, Rect},
};
use egui::FullOutput;
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
    last: Instant,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { sub: SubGui::new(), last: Instant::now(), }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();

            ui.heading("Boring host heading...");

            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), Sense::click_and_drag());

            let mut full_output: FullOutput = ui.ctx().input(|input_state| {
                let raw_input = convert_subwindow_input(input_state, rect);
                let input_bytes = bincode::serialize(&raw_input).unwrap();
                let output_bytes = self.sub.run(&input_bytes);

                let comp_size = output_bytes.len();
                let output_bytes =
                    quicklz::decompress(&mut Cursor::new(&output_bytes), 1024_u32.pow(3)).unwrap();
                let uncomp_size = output_bytes.len();

                let comp_ratio = uncomp_size as f32 / (comp_size as f32).max(1.0); 
                let fps = 1. / self.last.elapsed().as_secs_f32();
                let bps = fps * comp_size as f32;
                let mbps = bps * 8. / 1000. / 1000.;

                println!("ratio: {comp_ratio} size: {comp_size} speed {mbps} mbps");

                bincode::deserialize(&output_bytes).unwrap()
            });

            for ClippedShape { clip_rect, shape } in &mut full_output.shapes {
                let offset = rect.left_top().to_vec2();
                shape.translate(offset);
                ui.set_clip_rect(clip_rect.translate(offset));
                ui.painter().add(shape.clone());
            }
        });

        self.last = Instant::now();
    }
}

fn convert_subwindow_input(input_state: &InputState, rect: Rect) -> RawInput {
    let mut raw = input_state.raw.clone();
    for ev in &mut raw.events {
        match ev {
            Event::PointerMoved(new_pos) => {
                *new_pos -= rect.left_top().to_vec2();
            }
            Event::PointerButton { pos, .. } => {
                *pos -= rect.left_top().to_vec2();
            }
            _ => (),
        }
    }

    raw
}
