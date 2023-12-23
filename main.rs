use std::env;
use std::io;
use std::fs;
use std::fs::File;

fn usage(args: Vec<String>) {
    eprintln!("Usage: {} <file to visualize>", args[0]);
}

fn count_bytes(buffer: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut counts = vec![vec![0;256];256];
    for id in 0..(buffer.len() - 1) {
        let y = buffer[id];
        let x = buffer[id+1];
        counts[usize::from(y)][usize::from(x)] += 1;
    }

    let mut max:usize = 0;
    for y in counts.iter() {
        for &x in y.iter() {
            if x > max {
                max = x;
            }
        }
    }

    let max:f64 = max as f64;

    let mut normalized = vec![vec![0;256];256];

    for row in 0..256 {
        for col in 0..256 {
            normalized[row][col] = ((counts[row][col] as f64).log(max) * 255.0) as u8;
        }
    }

    normalized
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_path = &args[1];
        if let Ok(_) = File::open(&file_path) {
            let mut buffer:Vec<u8> = Vec::new();
            buffer.extend(fs::read(&file_path)?);
            let byte_counts = count_bytes(&buffer);
            for y in byte_counts.iter() {
                for &x in y.iter() {
                    print!("{} ", x);
                }
                print!("\n");
            }
            println!("");
        } else {
            eprintln!("Couldn't open file: {}", file_path);
        };
    } else {
        usage(args);
    };

    Ok(())
}
