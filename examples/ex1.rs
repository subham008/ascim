use ascim::{Arguments, AsciImage};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arguments {
        file_path: String::from("examples/image.jpg"),
        max_width: 80,
        max_height: 40,
        character_ratio: 2.0,
        edge_threshold: 1.0,
    };

    let ascii = AsciImage::from_args(&args);
    ascii.print();

    Ok(())
}
