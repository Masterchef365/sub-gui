use eframe::egui::{self, Context, FullOutput, RawInput, DragValue};
use egui_demo_lib::DemoWindows;
use quicklz::CompressionLevel;
pub struct SubGui {
    ctx: Context,
    demo: DemoWindows,
    compression_level: quicklz::CompressionLevel,
}

impl SubGui {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx, demo: DemoWindows::default(), compression_level: quicklz::CompressionLevel::Lvl1 }
    }

    pub fn run(&mut self, input_bytes: &[u8]) -> Vec<u8> {
        let raw_input = bincode::deserialize(input_bytes).unwrap();
        let full_output = self.ctx.run(raw_input, |ctx| {
            egui::SidePanel::left("leftpanel").show(ctx, |ui| {
                ui.label("Compression");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.compression_level, CompressionLevel::Lvl1, "Normal");
                    ui.selectable_value(&mut self.compression_level, CompressionLevel::Lvl3, "Ludicrous");
                });
            });

            ctx.request_repaint();
            self.demo.ui(ctx);
        });

        let uncompressed = bincode::serialize(&full_output).unwrap();
        quicklz::compress(&uncompressed, self.compression_level)
    }
}


