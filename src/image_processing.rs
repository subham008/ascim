use crate::argparse::Args;
use image::GenericImageView;

pub struct ImageData {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

pub fn load_and_resize_image(args: &Args) -> Result<ImageData, String> {
    let img = image::open(&args.file_path).map_err(|e| format!("Failed to load image: {}", e))?;

    let (original_width, original_height) = img.dimensions();

    let proposed_height = (original_height as f64 * args.max_width as f64)
        / (args.character_ratio * original_width as f64);

    let (width, height) = if proposed_height <= args.max_height as f64 {
        (args.max_width, proposed_height as usize)
    } else {
        let w = (args.character_ratio * original_width as f64 * args.max_height as f64)
            / original_height as f64;
        (w as usize, args.max_height)
    };

    let resized = img.resize_exact(
        width as u32,
        height as u32,
        image::imageops::FilterType::Triangle,
    );

    let rgb = resized.to_rgba8();
    let data = rgb.as_raw().clone();

    Ok(ImageData {
        width,
        height,
        data,
    })
}

pub fn make_grayscale(image: &ImageData) -> Vec<f64> {
    let mut grayscale = Vec::with_capacity(image.width * image.height);

    for i in 0..(image.width * image.height) {
        let idx = i * 4;
        let r = image.data[idx] as f64 / 255.0;
        let g = image.data[idx + 1] as f64 / 255.0;
        let b = image.data[idx + 2] as f64 / 255.0;

        let gray = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        grayscale.push(gray);
    }

    grayscale
}
