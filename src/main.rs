use clap::{Parser, Subcommand};
use image::{GenericImage, GenericImageView, ImageReader};
use std::fs;
use std::path::PathBuf;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode a message into an image
    E {
        /// Image file to encode the message into
        image: PathBuf,

        /// Message file to be encoded
        message: PathBuf,

        /// Output image file
        output: PathBuf,
    },
    /// Decode a message from an image
    D {
        /// Image file to decode the message from
        image: PathBuf,
    },
}

fn encode(image_path: &PathBuf, message: &PathBuf, output: &PathBuf) {
    let message = fs::read(message).expect("Should have been able to read the file");

    let length = format!("{:0>9b}", message.len());

    let combined_vec: Vec<u8> = length
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .chain(
            message
                .into_iter()
                .flat_map(|byte| (0..8).map(move |i| ((byte & (1 << (7 - i))) != 0) as u8)),
        )
        .collect(); // Collect everything into a single Vec<u8>

    let mut image = image::open(image_path).unwrap();
    let mut i = 0;
    let (w, h) = image.dimensions();
    for y in 0..h {
        for x in 0..w {
            let mut pixel = image.get_pixel(x, y);

            for j in 0..3 {
                if i >= combined_vec.len() {
                    pixel[j] = (pixel[j] & 0xFE) | combined_vec[i];
                }
                i += 1;
            }

            image.put_pixel(x, y, pixel);

            if i >= combined_vec.len() {
                break;
            }
        }
        if i >= combined_vec.len() {
            break;
        }
    }
    image.save("testing/output.png").unwrap();
    image.save(output).expect("not ok")
}

fn decode(image_path: &PathBuf) {
    // Load the image
    let image = ImageReader::open(image_path)
        .unwrap()
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();

    let mut bits = Vec::new();

    let mut mesage = Vec::new();
    let mut i = 0;
    'outer: for (_, _, pixel) in image.pixels() {
        println!("{:?}", pixel);
        for j in 0..3 {
            if bits.len() < 9 {
                bits.push(pixel[j] & 1);
                println!("message decoded!");
            } else {
                let length = bits.iter().take(9).fold(0u8, |acc, &bit| (acc << 1) | bit) as usize;
                if i >= 8 * length {
                    break 'outer;
                }
                mesage.push(pixel[j] & 1);
                i += 1;
            }
        }
    }
    // Convert bits into bytes
    let mut bytes = Vec::new();
    let mut byte = 0u8;
    for (i, bit) in mesage.clone().into_iter().enumerate() {
        byte = (byte << 1) | bit;
        if (i + 1) % 8 == 0 {
            bytes.push(byte);
            byte = 0;
        }
    }

    // Handle the case where the last byte might not be full
    if !bytes.is_empty() && (mesage.len() % 8) != 0 {
        bytes.push(byte);
    }

    println!("Decoded message: {:?}", String::from_utf8(bytes).unwrap());
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::E {
            image,
            message,
            output,
        } => encode(image, message, output),
        Commands::D { image } => decode(image),
    }
}
