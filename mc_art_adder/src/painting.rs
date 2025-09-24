// src/painting.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

impl Default for PaintingsList {
    fn default() -> Self {
        PaintingsList {
            schema: "https://example.com/painting-schema.json".to_string(),
            version: "1.0".to_string(),
            id: "default_id".to_string(),
            name: "Default Paintings List".to_string(),
            description: "A default list of paintings.".to_string(),
            paintings: Vec::new(),
        }
    }
}

impl PaintingsList {

    pub fn set_schema(&mut self, schema: String) {
        self.schema = schema;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn add_painting(&mut self, painting: Painting) {
        self.paintings.push(painting);
    }

}


impl Painting {

    pub fn new(id: String, source_name: String, output_path: String, aspect_data: [u32; 4]) -> Self {
        Painting {
            id,
            source_name,
            output_path,
            width: aspect_data[0],
            height: aspect_data[1],
        }  
    }

}