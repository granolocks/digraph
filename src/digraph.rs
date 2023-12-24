use image::{ImageBuffer, RgbImage};

/// Dimensions of the byte space as well as the dimensions of the digraph matrix
/// and resultant output png.
pub const SCALE : usize = 256;

/// File extension to append to whatever the original filename we received was
pub const IMG_EXT : &str = ".digraph.png";

/// Struct to hold info about the Digraph we are building
pub struct Digraph {

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
        let mut counts = vec![vec![0;SCALE];SCALE];

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
        let mut intensities = vec![vec![0;SCALE];SCALE];
        for row in 0..SCALE {
            for col in 0..SCALE {

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

    fn get_output_filename(&self) -> String {
        let mut outfile = self.base_name.clone();
        outfile.extend(IMG_EXT.chars());
        String::from(outfile)
    }

    /// Generate a png file from the computed intensities
    pub fn generate_png(&self) {
        let outfile : String = self.get_output_filename();
        let mut img: RgbImage = ImageBuffer::new(SCALE as u32, SCALE as u32);

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

