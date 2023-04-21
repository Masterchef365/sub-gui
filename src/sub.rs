use eframe::egui::{self, Context, FullOutput, RawInput};
pub struct SubGui {
    ctx: Context,
}

impl SubGui {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx }
    }

    pub fn run(&mut self, raw_input: RawInput) -> FullOutput {
        self.ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                ui.label("Hello world!");
                if ui.button("Click me").clicked() {
                    println!("I've been clicked aaaa");
                }
            });
        })
    }
}
