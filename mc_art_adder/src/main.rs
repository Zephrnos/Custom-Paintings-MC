// Declare the modules to link your other files
mod painting;
mod aspect_ratio;

// Bring the necessary items into scope from your other files
use crate::painting::{Painting, PaintingsList};
use crate::aspect_ratio::AspectRatio;

// Original use statements
use std::fs;
use std::path::Path;
use image::DynamicImage;
use image::GenericImageView;
use std::io;


// Function to get user input from the command line
fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Crop function that takes an image and crop data, returns the cropped image
fn crop(image: DynamicImage, crop_data: [u32; 4]) -> DynamicImage {
    image.crop_imm(crop_data[2], crop_data[3], crop_data[0], crop_data[1])
}


fn main() {

    // Creat an empty PaintingsList struct to fill in with data later
    let mut paintings_list = PaintingsList::default();

    // Get paintings_list data from user
    let schema = get_user_input("Enter schema URL:");
    let version = get_user_input("Enter version:");
    let id = get_user_input("Enter ID:");
    let name = get_user_input("Enter name:");
    let description = get_user_input("Enter description:");

    // Set the paintings_list data
    paintings_list.set_schema(schema);
    paintings_list.set_version(version);
    paintings_list.set_id(id);
    paintings_list.set_name(name);
    paintings_list.set_description(description);

    // Create output directory
    fs::create_dir_all("./output_dir").expect("Failed to create output directory");

    // Take in a directory of images and save the paths of each image to a vector
    fs::create_dir_all("./input_dir").expect("Failed to create output directory");

    /* Create the required file structures for the output directory
    
        current_dir/
            images.json
            output_dir/
                image1.png
                image2.png
                ...

     */ 
    todo!();
    
    /* For each image, spawn a rayon thread that does the following:
        - Open the image
        - For each aspect ratio in AspectRatio::ALL_RATIOS:
            - Get the crop data
            - Crop the image
            - Save the cropped image to the output directory with a name indicating the aspect ratio
            - Add an entry to images.json with the image name and aspect ratio and path
        - End the thread

    */
    todo!();
}





#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    // --- Tests for Square (1:1) Ratio ---

    #[test]
    fn test_square_crop_on_wide_image() {
        // Image: 1920x1080 (16:9), Target: 1:1, Limiting Dimension: Height
        let wide_image = DynamicImage::new_rgb8(1920, 1080);
        let crop_data = AspectRatio::Square.crop_data(&wide_image);
        // Expected: Centered 1080x1080 crop
        let expected_data: [u32; 4] = [1080, 1080, 420, 0];
        assert_eq!(crop_data, expected_data);
    }

    #[test]
    fn test_square_crop_on_tall_image() {
        // Image: 1080x1920 (9:16), Target: 1:1, Limiting Dimension: Width
        let tall_image = DynamicImage::new_rgb8(1080, 1920);
        let crop_data = AspectRatio::Square.crop_data(&tall_image);
        // Expected: Centered 1080x1080 crop
        let expected_data: [u32; 4] = [1080, 1080, 0, 420];
        assert_eq!(crop_data, expected_data);
    }

    #[test]
    fn test_square_crop_on_square_image() {
        // Image: 1000x1000 (1:1), Target: 1:1, Limiting Dimension: Either (no crop needed)
        let square_image = DynamicImage::new_rgb8(1000, 1000);
        let crop_data = AspectRatio::Square.crop_data(&square_image);
        // Expected: The original image dimensions
        let expected_data: [u32; 4] = [1000, 1000, 0, 0];
        assert_eq!(crop_data, expected_data);
    }

    // --- Tests for LongRectangle (4:3) Ratio ---

    #[test]
    fn test_long_rectangle_crop_on_wide_image() {
        // Image: 1235x865 (~1.42:1), Target: 4:3 (~1.33:1), Limiting Dimension: Height
        let wide_image = DynamicImage::new_rgb8(1235, 865);
        let crop_data = AspectRatio::LongRectangle.crop_data(&wide_image);
        // Expected: Centered 1153x865 crop
        let expected_data: [u32; 4] = [1152, 864, 41, 0];
        assert_eq!(crop_data, expected_data);
    }

    #[test]
    fn test_long_rectangle_crop_on_taller_image() {
        // Image: 900x1000 (0.9:1), Target: 4:3 (~1.33:1), Limiting Dimension: Width
        let taller_image = DynamicImage::new_rgb8(900, 1000);
        let crop_data = AspectRatio::LongRectangle.crop_data(&taller_image);
        // Expected: Centered 900x675 crop
        let expected_data: [u32; 4] = [900, 675, 0, 162];
        assert_eq!(crop_data, expected_data);
    }

    // --- Tests for Wide (2:1) Ratio ---

    #[test]
    fn test_wide_crop_on_narrower_image() {
        // Image: 1920x1080 (16:9), Target: 2:1, Limiting Dimension: Width
        let wide_image = DynamicImage::new_rgb8(1920, 1080);
        let crop_data = AspectRatio::Wide.crop_data(&wide_image);
        // Expected: Centered 1920x960 crop
        let expected_data: [u32; 4] = [1920, 960, 0, 60];
        assert_eq!(crop_data, expected_data);
    }

    #[test]
    fn test_wide_crop_on_wider_image() {
        // Image: 3000x1000 (3:1), Target: 2:1, Limiting Dimension: Height
        let wider_image = DynamicImage::new_rgb8(3000, 1000);
        let crop_data = AspectRatio::Wide.crop_data(&wider_image);
        // Expected: Centered 2000x1000 crop
        let expected_data: [u32; 4] = [2000, 1000, 500, 0];
        assert_eq!(crop_data, expected_data);
    }
    
    // --- Tests for TallRectangle (3:4) Ratio ---

    #[test]
    fn test_tall_rectangle_crop_on_wider_image() {
        // Image: 1000x1000 (1:1), Target: 3:4 (0.75:1), Limiting Dimension: Height
        let wide_image = DynamicImage::new_rgb8(1000, 1000);
        let crop_data = AspectRatio::TallRectangle.crop_data(&wide_image);
        // Expected: Centered 750x1000 crop
        let expected_data: [u32; 4] = [750, 1000, 125, 0];
        assert_eq!(crop_data, expected_data);
    }

    // --- Tests for Tall (1:2) Ratio ---

    #[test]
    fn test_tall_crop_on_wider_image() {
        // Image: 1000x1000 (1:1), Target: 1:2 (0.5:1), Limiting Dimension: Height
        let wide_image = DynamicImage::new_rgb8(1000, 1000);
        let crop_data = AspectRatio::Tall.crop_data(&wide_image);
        // Expected: Centered 500x1000 crop
        let expected_data: [u32; 4] = [500, 1000, 250, 0];
        assert_eq!(crop_data, expected_data);
    }

    #[test]
    fn test_tall_crop_on_taller_image() {
        // Image: 500x1500 (1:3), Target: 1:2 (0.5:1), Limiting Dimension: Width
        let taller_image = DynamicImage::new_rgb8(500, 1500);
        let crop_data = AspectRatio::Tall.crop_data(&taller_image);
        // Expected: Centered 500x1000 crop
        let expected_data: [u32; 4] = [500, 1000, 0, 250];
        assert_eq!(crop_data, expected_data);
    }
}

#[cfg(test)]
mod image_creation_tests {
    // (Your other use statements)
    use std::fs;
    use std::path::Path;
    use image::{Rgb, RgbImage};

    #[test]
    fn create_test_images_with_pattern() {
        // Define the checkerboard pattern properties
        const SQUARE_SIZE: u32 = 50;
        const COLOR_A: Rgb<u8> = Rgb([50, 100, 200]); // A shade of blue
        const COLOR_B: Rgb<u8> = Rgb([255, 200, 0]);  // A shade of yellow

        let images_to_create = vec![
            ("test_images/wide_image.jpg", 1920, 1080),
            ("test_images/tall_image.jpg", 1080, 1920),
            ("test_images/square_image.jpg", 1000, 1000),
            ("test_images/long_rectangle.jpg", 1235, 865),
            ("test_images/taller_image.jpg", 900, 1000),
            ("test_images/narrower_image.jpg", 1920, 1080),
            ("test_images/wider_image.jpg", 3000, 1000),
            ("test_images/wider2_image.jpg", 1000, 1000),
            ("test_images/taller2_image.jpg", 500, 1500),
        ];

        let output_dir = Path::new("test_images");
        if !output_dir.exists() {
            fs::create_dir(output_dir).expect("Failed to create test_images directory");
        }

        for (path_str, width, height) in images_to_create {
            let path = Path::new(path_str);
            if path.exists() {
                continue;
            }

            // Create a mutable image buffer.
            let mut img = RgbImage::new(width, height);

            // Iterate over each pixel to create the checkerboard pattern.
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let square_x = x / SQUARE_SIZE;
                let square_y = y / SQUARE_SIZE;

                // Alternate colors based on the square's position.
                if (square_x + square_y) % 2 == 0 {
                    *pixel = COLOR_A;
                } else {
                    *pixel = COLOR_B;
                }
            }
            
            img.save(path).expect(&format!("Failed to save image to {}", path_str));
            println!("Created test image with pattern: {}", path_str);
        }

        assert!(output_dir.exists());
    }
}