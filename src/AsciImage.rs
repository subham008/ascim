use crate::argparse::Args;
use crate::image_processing::{ImageData, load_and_resize_image, make_grayscale};

use crate::print_image::{
    get_ascii_char, get_color_code, get_sobel, get_sobel_angle_char, rgb_to_hsv,
};
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct RGBColor {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive(Debug, Clone)]
pub struct AsciImage {
    pub height: usize,
    pub width: usize,
    pub threshold: f64,
    pub character_ratio: f64,
    pub converted_image: Vec<Vec<(RGBColor, char)>>, // 2D array of converted image
}

impl AsciImage {
    /// Creates an AsciImage from a given file path with specified options.
    pub fn from_args(args: &Args) -> Self {
        let img_data = load_and_resize_image(&args).expect("Failed to load image");
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
