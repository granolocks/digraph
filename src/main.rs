use std::env;
use std::fs;
use std::path::Path;
use image::{ImageBuffer, RgbImage};

/// Dimensions of the byte space as well as the dimensions of the digraph matrix
/// and resultant output png.
const DIGRAPH_SCALE : usize = 256;

/// Struct to hold info about the Digraph we are building
struct Digraph {

    /// 2d Vec of bytes representing relative frequency of byte pairs in the 
    /// file processed where the byte pair is identified as its row / column
    /// position in the matrix
    intensities: Vec<Vec<u8>>,
    
    /// Name of the original processed file which will be used to create the name
    /// of the output PNG
    base_name: String
}

impl Digraph {

    /// Construct a Digraph from a basename and a raw buffer of bytes read from 
    /// some file (presumably named `base_name`).
    pub fn new(base_name: String, buffer: &Vec<u8>) -> Self {

        // Set up matrix to count byte frequency
        let mut counts = vec![vec![0;DIGRAPH_SCALE];DIGRAPH_SCALE];

        // Key track of highest byte frequency as we go
        let mut max:usize = 0;

        // iterate through the buffer skipping the last element since it has 
        // no preceding byte
        for id in 0..(buffer.len() - 1) {

            // x is the byte, y is the next byte. these are the coordinates
            // to increment in the counts matrix
            let x  = buffer[id];
            let y = buffer[id+1];

            //increment and update max if needed
            let new_val = counts[usize::from(y)][usize::from(x)] + 1;
            if new_val > max {
                max = new_val;
            }

            // set back into matrix 
            counts[usize::from(y)][usize::from(x)] = new_val;
        }
    
        // max -> f64 for val.log(max) purposes
        let max: f64 = max as f64;
        
        // set up a second matrix to copy the normalized intensities into
        let mut intensities = vec![vec![0;DIGRAPH_SCALE];DIGRAPH_SCALE];
        for row in 0..DIGRAPH_SCALE {
            for col in 0..DIGRAPH_SCALE {

                // normalize each count and then convert back into a relative 
                // u8 value to use as color intensity
                intensities[row][col] = ((counts[row][col] as f64).log(max) * 255.0) as u8;
            }
        }

        // return the thing
        Self { 
            intensities: intensities,
            base_name: base_name
        }
    }

    /// Generate a png file from the computed intensities
    pub fn generate_png(&self) {
        let mut outfile = String::new();
        outfile.extend(self.base_name.chars());
        outfile.extend(".digraph.png".chars());
        let mut img: RgbImage = ImageBuffer::new(DIGRAPH_SCALE as u32, DIGRAPH_SCALE as u32);

        // set computed relative frequencies into pixels
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let i = self.intensities[y as usize][x as usize];
            *pixel = image::Rgb([i, i, i]);
        }

        match img.save(&outfile) {
            Ok(_)  => println!("Saved image {outfile}"),
            Err(_) => eprintln!("Couldn't save image {outfile}")
        }
    }
}

/// Prints usage
fn usage(args: &Vec<String>) {
    eprintln!("Usage: {} <file to visualize>", args[0]);
}

/// Read a file into a byte vec
fn read_file_to_bytes(filepath: &Path) -> Option<Vec<u8>> {
    if let Ok(buffer) = fs::read(&filepath) {
        Some(buffer)
    } else {
        None
    }
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    // Currently there must be exactly 2 args: <progname> <file to process>
    if args.len() != 2 {
        usage(&args);
        return Err("Invalid args.");
    }

    let filepath = Path::new(&args[1]);
    
    match read_file_to_bytes(&filepath) {
        Some(buffer) => {
            let filename = filepath.file_name().unwrap().to_str().unwrap();
            let digraph = Digraph::new(String::from(filename), &buffer);
            digraph.generate_png();
        },
        None => {
            usage(&args);
            eprintln!("Bad file {}", &filepath.display());
            return Err("Couldn't read file");
        }
    }

    Ok(())
}
