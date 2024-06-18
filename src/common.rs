use egui::{epaint::ClippedShape, FullOutput};

use std::collections::HashMap;

use crate::hash_abuse::{EqByHash, HashBySerialize};

enum UpdateData {
    FullUpdate(FullOutput),
    /// Used to build a FullOutput on the other side
    Partial(FullOutput, Vec<PartialUpdate>),
}

enum PartialUpdate {
    Reference(usize),
    Shape(ClippedShape),
}

struct Encoder {
    memory: HashMap<EqByHash<HashBySerialize<ClippedShape>>, usize>,
}

struct Decoder {
    memory: Option<FullOutput>,
}

impl Encoder {
    pub fn new() -> Self {
        Self {
            memory: Default::default(),
        }
    }

    pub fn encode(&mut self, data: &FullOutput, partial: bool) -> UpdateData {
        if partial {
            let mut data = data.clone();
            let partial_updates = data
                .shapes
                .drain(..)
                .map(|shape| {
                    let casted = EqByHash(HashBySerialize(shape));
                    if let Some(&index) = self.memory.get(&casted) {
                        PartialUpdate::Reference(index)
                    } else {
                        PartialUpdate::Shape(casted.0 .0)
                    }
                })
                .collect();

            UpdateData::Partial(data, partial_updates)
        } else {
            UpdateData::FullUpdate(data.clone())
        }
    }

    fn encode_partial(&self, data: &FullOutput) -> UpdateData {}
}

impl Decoder {
    pub fn new() -> Self {
        Self { memory: None }
    }
}
