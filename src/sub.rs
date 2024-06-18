use std::{hash::Hash, io::Write};

use eframe::egui::{self, Context, FullOutput, RawInput, DragValue};
use egui::{ahash::HashSet, epaint::ClippedShape};
use egui_demo_lib::DemoWindows;
use quicklz::CompressionLevel;

pub struct SubGui {
    ctx: Context,
    demo: DemoWindows,
    compression_level: quicklz::CompressionLevel,
    last: HashSet<EqByHash<HashBySerialize<ClippedShape>>>,
}

impl SubGui {
    pub fn new() -> Self {
        let ctx = Context::default();
        Self { ctx, demo: DemoWindows::default(), compression_level: quicklz::CompressionLevel::Lvl1, last: Default::default() }
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

        let count = full_output.shapes.len();
        self.last = full_output.shapes.iter().map(|c| EqByHash(HashBySerialize(c.clone()))).collect();

        //println!("{}", serde_json::ser::to_string_pretty(&full_output).unwrap());

        let last_len = self.last.len();
        dbg!(last_len, count);

        let uncompressed = bincode::serialize(&full_output).unwrap();
        quicklz::compress(&uncompressed, self.compression_level)
    }
}


#[derive(PartialEq, Eq)]
struct HashBySerialize<T>(pub T);

impl<T: serde::Serialize> Hash for HashBySerialize<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        bincode::serialize_into(HashWriter(state), &self.0).unwrap();
    }
}

struct HashWriter<T>(pub T);

impl<T: std::hash::Hasher> Write for HashWriter<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[derive(Hash)]
struct EqByHash<T>(T);

use std::hash::Hasher;

impl<T: Hash> PartialEq<EqByHash<T>> for EqByHash<T> {
    fn eq(&self, other: &EqByHash<T>) -> bool {
        let mut hash_self = std::hash::DefaultHasher::new();
        self.0.hash(&mut hash_self);

        let mut hash_other = std::hash::DefaultHasher::new();
        other.0.hash(&mut hash_other);

        hash_self.finish() == hash_other.finish()
    }
}

impl<T: Hash> Eq for EqByHash<T> {}
