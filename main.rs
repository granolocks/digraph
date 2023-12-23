use std::env;
use std::io;
use std::fs;
use std::fs::File;
use std::path::Path;
use image::{ImageBuffer, RgbImage};

struct Digraph {
    intensities: Vec<Vec<u8>>,
    base_name: String
}

impl Digraph {
    pub fn new(base_name: String, buffer: &Vec<u8>) -> Self {
        let mut counts = vec![vec![0;256];256];
        let mut max:usize = 0;
        for id in 0..(buffer.len() - 1) {
            let x  = buffer[id];
            let y = buffer[id+1];

            let new_val = counts[usize::from(y)][usize::from(x)] + 1;
            if new_val > max {
                max= new_val;
            }

            counts[usize::from(y)][usize::from(x)] = new_val;
        }
        let max:f64 = max as f64;
        let mut intensities = vec![vec![0;256];256];
        for row in 0..256 {
            for col in 0..256 {
                intensities[row][col] = ((counts[row][col] as f64).log(max) * 255.0) as u8;
            }
        }

        Self { 
            intensities: intensities,
            base_name: base_name
        }
    }
}

fn usage(args: Vec<String>) {
    eprintln!("Usage: {} <file to visualize>", args[0]);
}

fn make_png(digraph: &Digraph) {
    let mut outfile = String::new();
    outfile.extend(digraph.base_name.chars());
    outfile.extend(".digraph.png".chars());
    let mut img: RgbImage = ImageBuffer::new(256, 256);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let i = digraph.intensities[y as usize][x as usize];
        *pixel = image::Rgb([i, i, i]);
    }

    match img.save(&outfile) {
        Ok(_)  => println!("Saved image {outfile}"),
        Err(_) => eprintln!("Couldn't save image {outfile}")
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let file_path = Path::new(&args[1]);
        if let Ok(_) = File::open(&file_path) {
            let filename = file_path.file_name().unwrap().to_str().unwrap();
            let mut buffer:Vec<u8> = Vec::new();
            buffer.extend(fs::read(&file_path)?);

            let digraph = Digraph::new(String::from(filename), &buffer);
            make_png(&digraph);

        } else {
            eprintln!("Couldn't open file: {}", file_path.display());
        };
    } else {
        usage(args);
    };

    Ok(())
}
