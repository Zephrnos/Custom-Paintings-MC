// src/painting.rs

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaintingsList {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub id: String,
    pub name: String,
    pub description: String,
    pub paintings: Vec<Painting>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Painting {
    pub id: String,
    pub filename: String, // <-- ADDED: To link to the image file
    pub name: String,
    pub artist: String,
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
    pub fn set_schema(&mut self, schema: String) { self.schema = schema; }
    pub fn set_version(&mut self, version: String) { self.version = version; }
    pub fn set_id(&mut self, id: String) { self.id = id; }
    pub fn set_name(&mut self, name: String) { self.name = name; }
    pub fn set_description(&mut self, description: String) { self.description = description; }
    pub fn add_painting(&mut self, painting: Painting) { self.paintings.push(painting); }
}

impl Painting {
    // MODIFIED: The function signature is updated to build the painting correctly.
    pub fn new(
        original_filename: String,
        width: u32,
        height: u32,
        painting_id: String,
        image_filename: String, // <-- ADDED parameter
    ) -> Self {
        let file_stem = Path::new(&original_filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        if let Some((name_part, artist_part)) = file_stem.rsplit_once("_drawn_by_") {
            let name = name_part.replace('_', " ");
            let artist_name_only = artist_part.split("__").next().unwrap_or(artist_part);
            let artist = artist_name_only.replace('_', " ");

            return Self {
                id: painting_id,
                filename: image_filename, // <-- SET the new filename field
                name,
                artist,
                width,  // Use the passed-in width
                height, // Use the passed-in height
            };
        }

        Painting::default()
    }
}