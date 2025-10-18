use std::env;

const DEFAULT_MAX_WIDTH: usize = 64;
const DEFAULT_MAX_HEIGHT: usize = 48;
const DEFAULT_CHARACTER_RATIO: f64 = 2.0;
const DEFAULT_EDGE_THRESHOLD: f64 = 4.0;

pub struct Args {
    pub file_path: String,
    pub max_width: usize,
    pub max_height: usize,
    pub character_ratio: f64,
    pub edge_threshold: f64,
}

impl Args {
    pub fn parse() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            Self::print_help(&args[0]);
            return Args {
                file_path: String::new(),
                max_width: DEFAULT_MAX_WIDTH,
                max_height: DEFAULT_MAX_HEIGHT,
                character_ratio: DEFAULT_CHARACTER_RATIO,
                edge_threshold: DEFAULT_EDGE_THRESHOLD,
            };
        }

        if args[1] == "-h" || args[1] == "--help" {
            Self::print_help(&args[0]);
            return Args {
                file_path: String::new(),
                max_width: DEFAULT_MAX_WIDTH,
                max_height: DEFAULT_MAX_HEIGHT,
                character_ratio: DEFAULT_CHARACTER_RATIO,
                edge_threshold: DEFAULT_EDGE_THRESHOLD,
            };
        }

        let mut parsed_args = Args {
            file_path: args[1].clone(),
            max_width: Self::get_terminal_width().unwrap_or(DEFAULT_MAX_WIDTH),
            max_height: Self::get_terminal_height().unwrap_or(DEFAULT_MAX_HEIGHT),
            character_ratio: DEFAULT_CHARACTER_RATIO,
            edge_threshold: DEFAULT_EDGE_THRESHOLD,
        };

        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "-mw" if i + 1 < args.len() => {
                    if let Ok(width) = args[i + 1].parse() {
                        parsed_args.max_width = width;
                    }
                    i += 2;
                }
                "-mh" if i + 1 < args.len() => {
                    if let Ok(height) = args[i + 1].parse() {
                        parsed_args.max_height = height;
                    }
                    i += 2;
                }
                "-et" if i + 1 < args.len() => {
                    if let Ok(threshold) = args[i + 1].parse() {
                        parsed_args.edge_threshold = threshold;
                    }
                    i += 2;
                }
                "-cr" if i + 1 < args.len() => {
                    if let Ok(ratio) = args[i + 1].parse() {
                        parsed_args.character_ratio = ratio;
                    }
                    i += 2;
                }
                _ => i += 1,
            }
        }

        parsed_args
    }

    fn print_help(exec_name: &str) {
        println!("USAGE:");
        println!("\t{} <path/to/image> [OPTIONS]\n", exec_name);
        println!("ARGUMENTS:");
        println!("\t<path/to/image>\t\tPath to image file\n");
        println!("OPTIONS:");
        println!(
            "\t-mw <width>\t\tMaximum width in characters (default: terminal width OR {})",
            DEFAULT_MAX_WIDTH
        );
        println!(
            "\t-mh <height>\t\tMaximum height in characters (default: terminal height OR {})",
            DEFAULT_MAX_HEIGHT
        );
        println!(
            "\t-et <threshold>\t\tEdge detection threshold, range: 0.0 - 4.0 (default: {:.1}, disabled)",
            DEFAULT_EDGE_THRESHOLD
        );
        println!(
            "\t-cr <ratio>\t\tHeight-to-width ratio for characters (default: {:.1})",
            DEFAULT_CHARACTER_RATIO
        );
    }

    fn get_terminal_width() -> Option<usize> {
        if let Some((w, _)) = term_size::dimensions() {
            Some(w)
        } else {
            None
        }
    }

    fn get_terminal_height() -> Option<usize> {
        if let Some((_, h)) = term_size::dimensions() {
            Some(h)
        } else {
            None
        }
    }
}
