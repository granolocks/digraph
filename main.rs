use std::env;

fn usage(args: Vec<String>) {
    println!("Usage: {} <file to visualize>", args[0]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_path = &args[1];
        dbg!(file_path);
    } else {
        usage(args);
    };
}
