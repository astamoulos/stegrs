use clap::Parser;
use image::GenericImageView;
use std::fs;

// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // The pattern to look for
    image: std::path::PathBuf,
    // The path to the file to read
    message: std::path::PathBuf,
    // The output image
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    
    let message = fs::read(&args.message)
        .expect("Should have been able to read the file");

    let img = image::open(args.image).unwrap();
    println!("dimensions {:?}", img.dimensions());
    //println!("path: {:?}, message: {:?}", args.message, img);

    let mut bits = Vec::new(); 
    for byte in message {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1);
        }
    }

    println!("{:?}", bits.len());
    let mut i = 0; 

    for (_, _, mut pixel) in img.pixels() {
        if i >=  bits.len() {
            println!("message encoded!a");
            break;
        }
        println!("{}", i);
        pixel[0] = (pixel[0] & 0xFE) | bits[i];
        i = i + 1;
        if i >=  bits.len() {
            println!("message encoded!a");
            break;
        }
        pixel[1] = (pixel[1] & 0xFE) | bits[i];

        i = i + 1;

        if i >=  bits.len() {
            println!("message encoded!a");
            break;
        }
        pixel[2] = (pixel[2] & 0xFE) | bits[i];
        
        i = i + 1;
    }
    let _ = img.save(args.output);
}
