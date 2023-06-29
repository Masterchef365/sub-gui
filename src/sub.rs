use eframe::egui::{self, Context, FullOutput, RawInput, DragValue};
pub struct SubGui {
    ctx: Context,
    frunge: f32,
}

impl SubGui {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx, frunge: 0.0 }
    }

    pub fn run(&mut self, input_bytes: &[u8]) -> Vec<u8> {
        let raw_input = bincode::deserialize(input_bytes).unwrap();
        let full_output = self.ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.label("Hello world!");
                if ui.button("Click me").clicked() {
                    println!("I've been clicked aaaa");
                }
                ui.add(DragValue::new(&mut self.frunge));
            });
        });

        bincode::serialize(&full_output).unwrap()
    }
}
