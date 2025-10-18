mod argparse;
mod image_processing;
mod print_image;

use argparse::Args;
use image_processing::load_and_resize_image;
use print_image::print_image;

fn main() {
    let args = Args::parse();

    if args.file_path.is_empty() {
        eprintln!("Error: No file path provided");
        std::process::exit(1);
    }

    match load_and_resize_image(&args) {
        Ok(image_data) => {
            print_image(&image_data, args.edge_threshold);
        }
        Err(e) => {
            eprintln!("Error loading image: {}", e);
            std::process::exit(1);
        }
    }
}
