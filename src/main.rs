use clap::Parser;
use std::io::Cursor;
use image::ImageReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    image: std::path::PathBuf,
    /// The path to the file to read
    message: std::path::PathBuf,
    /// The output image
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    let img = ImageReader::open(args.image)?.decode()?;

    println!("pattern: {:?}, path: {:?}", args.message, args.output)
}
