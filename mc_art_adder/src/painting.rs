// src/painting.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PaintingsList {
    pub schema: String,
    pub version: String,
    pub id: String,
    pub name: String,
    pub description: String,
    pub paintings: Vec<Painting>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Painting {
    pub id: String,
    pub source_name: String,
    pub output_path: String,
    pub width: u32,
    pub height: u32,
}

trait PaintingsData {
    fn set_id(&mut self, id: String);
    fn set_name(&mut self, name: String);
}

impl PaintingsData for Painting {
    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn set_name(&mut self, name: String) {
        self.source_name = name;
    }
}
impl PaintingsData for PaintingsList {
    fn set_id(&mut self, id: String) {
        self.id = id;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl PaintingsList {

}

impl Painting {

    fn set_output_path() {
        todo!()
    }

    fn set_ratio_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}