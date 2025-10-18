//! Crate-level documentation pulled from README
#![doc = include_str!("../README.md")]

mod argparse;
mod image_processing;
mod print_image;

use argparse::Args;
use image_processing::{ImageData, load_and_resize_image, make_grayscale};

use print_image::{get_ascii_char, get_sobel, get_sobel_angle_char, rgb_to_hsv};
use std::f64::consts::PI;

/// Represents an RGB color with floating point values between 0.0 and 1.0
///
/// # Fields
/// * `r` - Red component (0.0 to 1.0)
/// * `g` - Green component (0.0 to 1.0)
/// * `b` - Blue component (0.0 to 1.0)
#[derive(Debug, Clone, Copy)]
pub struct RGBColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

///
/// * `file_path` - Path to the input image file
/// * `max_width` - Maximum width of the output ASCII art in characters
/// * `max_height` - Maximum height of the output ASCII art in characters
/// * `character_ratio` - Aspect ratio correction for terminal characters (typically 2.0)
/// * `edge_threshold` - Threshold for edge detection (range: 0.0 to 4.0)
///
pub struct Arguments {
    pub file_path: String,
    pub max_width: usize,
    pub max_height: usize,
    pub character_ratio: f64,
    pub edge_threshold: f64,
}

/// Represents an ASCII art image with color information
///
/// # Fields
/// * `height` - Height of the image in characters
/// * `width` - Width of the image in characters  
/// * `threshold` - Edge detection threshold value
/// * `character_ratio` - Aspect ratio correction for terminal characters
/// * `converted_image` - 2D vector containing RGB colors and ASCII characters
#[derive(Debug, Clone)]
pub struct AsciImage {
    pub height: usize,
    pub width: usize,
    pub threshold: f64,
    pub character_ratio: f64,
    pub converted_image: Vec<Vec<(RGBColor, char)>>, // 2D array of converted image
}

impl AsciImage {
    /// Creates a new AsciImage instance from command line arguments
    ///
    /// # Arguments
    /// * `args` - Command line arguments containing image path and conversion options
    ///
    /// # Returns
    /// * `AsciImage` - A new AsciImage instance with the converted image data
    ///
    ///  # Example
    /// ```rust
    /// let args = Arguments {
    ///     file_path: String::from("path/to/image.jpg"),
    ///     max_width: 80,
    ///     max_height: 40,
    ///     character_ratio: 2.0,
    ///     edge_threshold: 1.0,
    /// };
    ///
    /// # Panics
    /// * If the image fails to load or process
    pub fn from_args(args: &Arguments) -> Self {
        let img_data = load_and_resize_image(&Args {
            file_path: args.file_path.clone(),
            max_width: args.max_width,
            max_height: args.max_height,
            character_ratio: args.character_ratio,
            edge_threshold: args.edge_threshold,
        })
        .expect("Failed to load image");
        let threshold = args.edge_threshold;
        let character_ratio = args.character_ratio;
        let converted_image = Self::convert_to_ascii(&img_data, threshold);

        AsciImage {
            height: img_data.height,
            width: img_data.width,
            threshold,
            character_ratio,
            converted_image,
        }
    }

    /// Prints the ASCII art with ANSI colors to the console
    ///
    /// # Example
    /// ```rust
    /// use ascim::AsciImage;
    ///
    /// let args = Arguments {
    ///     file_path: String::from("examples/image.jpg"),
    ///     max_width: 80,
    ///     max_height: 40,
    ///     character_ratio: 2.0,
    ///     edge_threshold: 1.0,
    /// };
    ///
    /// let ascii = AsciImage::from_args(&args);
    /// ascii.print; // Prints the colored ASCII art to console
    /// ```
    pub fn print(&self) {
        for row in &self.converted_image {
            for &(color, ch) in row {
                // Convert RGB values (0.0-1.0) to 0-255 range
                let r = (color.r * 255.0) as u8;
                let g = (color.g * 255.0) as u8;
                let b = (color.b * 255.0) as u8;

                // Print colored character using ANSI escape codes
                print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch);
            }
            println!();
        }
    }

    /// Converts the image data to ASCII characters based on the threshold and character ratio.
    fn convert_to_ascii(image_data: &ImageData, threshold: f64) -> Vec<Vec<(RGBColor, char)>> {
        let grayscale = make_grayscale(image_data);

        let (sobel_x, sobel_y) = if threshold < 4.0 {
            get_sobel(&grayscale, image_data.width, image_data.height)
        } else {
            (
                vec![0.0; image_data.width * image_data.height],
                vec![0.0; image_data.width * image_data.height],
            )
        };

        // Initialize the ascii_image with default values
        let mut ascii_image = vec![
            vec![
                (
                    RGBColor {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0
                    },
                    ' '
                );
                image_data.width
            ];
            image_data.height
        ];

        for y in 0..image_data.height {
            for x in 0..image_data.width {
                let idx = y * image_data.width + x;
                let pixel_idx = idx * 4;

                let r = image_data.data[pixel_idx] as f64 / 255.0;
                let g = image_data.data[pixel_idx + 1] as f64 / 255.0;
                let b = image_data.data[pixel_idx + 2] as f64 / 255.0;

                let r_f = r as f64 / 255.0;
                let g_f = g as f64 / 255.0;
                let b_f = b as f64 / 255.0;

                let hsv = rgb_to_hsv(r_f, g_f, b_f);
                let grayscale_val = hsv.value * hsv.value;

                let sx = sobel_x[idx];
                let sy = sobel_y[idx];
                let square_sobel_magnitude = sx * sx + sy * sy;

                let ascii_char = if square_sobel_magnitude >= threshold * threshold {
                    let sobel_angle = sy.atan2(sx) * 180.0 / PI;
                    get_sobel_angle_char(sobel_angle)
                } else {
                    get_ascii_char(grayscale_val)
                };

                // Store the color and character in the ascii_image array
                ascii_image[y][x] = (RGBColor { r, g, b }, ascii_char);
            }
        }

        ascii_image
    }
}
