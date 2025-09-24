// src/painting.rs

use crate::aspect_ratio::AspectRatio; // Import AspectRatio
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct PaintingsList {
    #[serde(rename = "$schema")] // Renames 'schema' to '$schema' in JSON output
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
    // This function is completely rewritten
    pub fn new(filename: String, aspect_ratio: AspectRatio) -> Self {
        // Get block dimensions from the aspect ratio
        let (width, height) = aspect_ratio.block_dimensions();

        // Get the filename without the extension (e.g., "Mona_Lisa_drawn_by_Leonardo_da_Vinci")
        let file_stem = Path::new(&filename)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();

        // Split the name and artist based on "_drawn_by_"
        if let Some((name_part, artist_part)) = file_stem.rsplit_once("_drawn_by_") {
            // Convert underscores to spaces for display
            let name = name_part.replace('_', " ");
            let artist = artist_part.replace('_', " ");

            // Create the JSON ID (e.g., "davinci_mona_lisa")
            // This takes the last name of the artist for the ID.
            let artist_id_part = artist
                .split_whitespace()
                .last()
                .unwrap_or("")
                .to_lowercase();
                
            let name_id_part = name_part.to_lowercase();
            let id = format!("{}_{}", artist_id_part, name_id_part);

            return Self { id, name, artist, width, height, };
        }

        // If the filename does not match the format, return a default Painting.
        Painting::default()
    }
}