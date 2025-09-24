// src/aspect_ratio.rs

use image::{DynamicImage, GenericImageView};

#[derive(Debug, Clone, Copy)]
pub enum AspectRatio {
    Square,
    Wide,
    LongRectangle,
    Tall,
    TallRectangle,
}

impl AspectRatio {
    pub const ALL_RATIOS: [Self; 5] = [
        Self::Square,
        Self::Wide,
        Self::LongRectangle,
        Self::Tall,
        Self::TallRectangle,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            AspectRatio::Square => "square",
            AspectRatio::Wide => "wide",
            AspectRatio::LongRectangle => "long_rectangle",
            AspectRatio::Tall => "tall",
            AspectRatio::TallRectangle => "tall_rectangle",
        }
    }

    pub fn crop_data(&self, image: &DynamicImage) -> [u32; 4] {
        let mut crop_dimensions: [u32; 4] = [0; 4];
        let (img_x, img_y) = image.dimensions();
        let (ratio_x, ratio_y) = match self {
            AspectRatio::Square => (1, 1),
            AspectRatio::Wide => (2, 1),
            AspectRatio::LongRectangle => (4, 3),
            AspectRatio::Tall => (1, 2),
            AspectRatio::TallRectangle => (3, 4),
        };
        let is_crop_wider =
            (ratio_x as f32 / ratio_y as f32) >= (img_x as f32 / img_y as f32);
        let magic_number: u32 = match is_crop_wider {
            true => img_x / ratio_x,
            false => img_y / ratio_y,
        };
        crop_dimensions[0] = magic_number * ratio_x;
        crop_dimensions[1] = magic_number * ratio_y;
        crop_dimensions[2] = (img_x - crop_dimensions[0]) / 2;
        crop_dimensions[3] = (img_y - crop_dimensions[1]) / 2;
        crop_dimensions
    }
}