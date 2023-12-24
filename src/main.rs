use std::env;
use std::path::Path;

mod digraph;
mod file_utils;

/// Prints usage
fn usage(args: &Vec<String>) {
    eprintln!("Usage: {} <file to visualize>", args[0]);
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    // Currently there must be exactly 2 args: <progname> <file to process>
    if args.len() != 2 {
        usage(&args);
        return Err("Invalid args");
    }

    let filepath = Path::new(&args[1]);
    match file_utils::read_file_to_bytes(&filepath) {
        Some((filename, buffer)) => {
             digraph::Digraph::new(filename, &buffer).generate_png();
             Ok(())
        },
        None => {
            usage(&args);
            eprintln!("Bad file {}", &filepath.display());
            Err("Couldn't read file")
        }
    }
}
