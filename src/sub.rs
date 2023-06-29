use eframe::egui::{self, Context, FullOutput, RawInput, DragValue};
pub struct SubGui {
    ctx: Context,
    frunge: f64,
}

impl SubGui {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx, frunge: 1.0 }
    }

    pub fn run(&mut self, input_bytes: &[u8]) -> Vec<u8> {
        let raw_input = bincode::deserialize(input_bytes).unwrap();
        let full_output = self.ctx.run(raw_input, |ctx| {
            egui::CentralPanel::default().show(&ctx, |ui| {
                use egui::plot::{Line, Plot, PlotPoints};
                let sin: PlotPoints = (0..1000).map(|i| {
                    let x = i as f64 * 0.01;
                    [x, (x * self.frunge).sin()]
                }).collect();
                let line = Line::new(sin);
                Plot::new("my_plot").view_aspect(2.0).show(ui, |plot_ui| plot_ui.line(line));

                ui.label("Hello world!");
                if ui.button("Click me").clicked() {
                    println!("I've been clicked aaaa");
                }
                ui.add(DragValue::new(&mut self.frunge).speed(1e-2));
            });
        });

        bincode::serialize(&full_output).unwrap()
    }
}
