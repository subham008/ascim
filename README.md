# ascim

A small Rust command-line tool and library for converting images to ASCII art. Useful for quick previews, terminal wallpapers, demos, or embedding simple text-based representations of images.

## Features
- Convert images to ASCII art with configurable width and character set
- Support for color output (ANSI) and plain monochrome output
- Library API for integration into other Rust projects
- Fast, memory-efficient processing using Rust image crates

## Installation
```bash
cargo install ascim
```

## Usage
Basic CLI usage:
```bash
# convert image.png to ASCII on stdout
ascim convert image.png --width 80

# save colored output to a file
ascim convert image.png --width 120 --color > image.asc
```

Library example:
```rust
use ascim::{Config, convert_from_path};

let cfg = Config::builder().width(100).color(true).build();
let ascii = convert_from_path("image.png", &cfg)?;
println!("{}", ascii);
```

## Configuration
Common options:
- width: target output width in characters
- color: enable ANSI color output
- charset: character set to use for density mapping
- invert: invert brightness mapping


## Contributing
Contributions are welcome. Open issues for bugs or feature requests and submit PRs for fixes or improvements. Keep changes small and include tests where applicable.

## License
MIT OR Apache-2.0 — see LICENSE file for details.