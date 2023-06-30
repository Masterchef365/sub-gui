use eframe::egui::{self, Context, FullOutput, RawInput, DragValue};
use egui_demo_lib::DemoWindows;
pub struct SubGui {
    ctx: Context,
    demo: DemoWindows,
}

impl SubGui {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx, demo: DemoWindows::default() }
    }

    pub fn run(&mut self, input_bytes: &[u8]) -> Vec<u8> {
        let raw_input = bincode::deserialize(input_bytes).unwrap();
        let full_output = self.ctx.run(raw_input, |ctx| {
            ctx.request_repaint();
            self.demo.ui(ctx);
        });

        bincode::serialize(&full_output).unwrap()
    }
}
