use crate::image_processing::{ImageData, make_grayscale};
use std::f64::consts::PI;

const VALUE_CHARS: &str = " .-=+*x#$&X@";

#[derive(Clone, Copy)]
pub struct Hsv {
    hue: f64,
    saturation: f64,
    pub value: f64,
}

pub fn rgb_to_hsv(red: f64, green: f64, blue: f64) -> Hsv {
    let max = red.max(green).max(blue);
    let min = red.min(green).min(blue);

    let value = max;
    let chroma = max - min;

    let saturation = if value.abs() < 1e-4 {
        0.0
    } else {
        chroma / value
    };

    let hue = if chroma < 1e-4 {
        0.0
    } else if (max - red).abs() < 1e-10 {
        let h = 60.0 * ((green - blue) / chroma % 6.0);
        if h < 0.0 { h + 360.0 } else { h }
    } else if (max - green).abs() < 1e-10 {
        60.0 * (2.0 + (blue - red) / chroma)
    } else {
        60.0 * (4.0 + (red - green) / chroma)
    };

    Hsv {
        hue,
        saturation,
        value,
    }
}

pub fn get_color_code(hsv: &Hsv) -> &'static str {
    if hsv.saturation < 0.25 {
        return "\x1B[37m"; // WHITE
    }

    if hsv.hue >= 30.0 && hsv.hue < 90.0 {
        "\x1B[33m" // YELLOW
    } else if hsv.hue >= 90.0 && hsv.hue < 150.0 {
        "\x1B[32m" // GREEN
    } else if hsv.hue >= 150.0 && hsv.hue < 210.0 {
        "\x1B[36m" // CYAN
    } else if hsv.hue >= 210.0 && hsv.hue < 270.0 {
        "\x1B[34m" // BLUE
    } else if hsv.hue >= 270.0 && hsv.hue < 330.0 {
        "\x1B[35m" // MAGENTA
    } else {
        "\x1B[31m" // RED
    }
}

pub fn get_ascii_char(grayscale: f64) -> char {
    let index = (grayscale * (VALUE_CHARS.len() - 1) as f64) as usize;
    let index = index.min(VALUE_CHARS.len() - 1);
    VALUE_CHARS.chars().nth(index).unwrap()
}

pub fn get_sobel_angle_char(sobel_angle: f64) -> char {
    if (22.5..=67.5).contains(&sobel_angle) || (-157.5..=-112.5).contains(&sobel_angle) {
        '\\'
    } else if (67.5..=112.5).contains(&sobel_angle) || (-112.5..=-67.5).contains(&sobel_angle) {
        '_'
    } else if (112.5..=157.5).contains(&sobel_angle) || (-67.5..=-22.5).contains(&sobel_angle) {
        '/'
    } else {
        '|'
    }
}

pub fn get_sobel(grayscale: &[f64], width: usize, height: usize) -> (Vec<f64>, Vec<f64>) {
    let gx = [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    let gy = [1.0, 2.0, 1.0, 0.0, 0.0, 0.0, -1.0, -2.0, -1.0];

    let mut sobel_x = vec![0.0; width * height];
    let mut sobel_y = vec![0.0; width * height];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;

            for j in -1..=1 {
                for i in -1..=1 {
                    let px = (x as i32 + i) as usize;
                    let py = (y as i32 + j) as usize;
                    let idx = py * width + px;
                    let kernel_idx = ((j + 1) * 3 + (i + 1)) as usize;

                    sum_x += gx[kernel_idx] * grayscale[idx];
                    sum_y += gy[kernel_idx] * grayscale[idx];
                }
            }

            let idx = y * width + x;
            sobel_x[idx] = sum_x;
            sobel_y[idx] = sum_y;
        }
    }

    (sobel_x, sobel_y)
}

pub fn print_image(image: &ImageData, edge_threshold: f64) {
    let grayscale = make_grayscale(image);
    let (sobel_x, sobel_y) = if edge_threshold < 4.0 {
        get_sobel(&grayscale, image.width, image.height)
    } else {
        (
            vec![0.0; image.width * image.height],
            vec![0.0; image.width * image.height],
        )
    };

    for y in 0..image.height {
        for x in 0..image.width {
            let idx = y * image.width + x;
            let pixel_idx = idx * 4;

            let r = image.data[pixel_idx] as f64 / 255.0;
            let g = image.data[pixel_idx + 1] as f64 / 255.0;
            let b = image.data[pixel_idx + 2] as f64 / 255.0;

            let hsv = rgb_to_hsv(r, g, b);
            let grayscale_val = hsv.value * hsv.value;
            let color = get_color_code(&hsv);

            let sx = sobel_x[idx];
            let sy = sobel_y[idx];
            let square_sobel_magnitude = sx * sx + sy * sy;

            let ascii_char = if square_sobel_magnitude >= edge_threshold * edge_threshold {
                let sobel_angle = sy.atan2(sx) * 180.0 / PI;
                get_sobel_angle_char(sobel_angle)
            } else {
                get_ascii_char(grayscale_val)
            };

            print!("{}{}", color, ascii_char);
        }
        println!();
    }

    print!("\x1B[0m"); // Reset color
}
