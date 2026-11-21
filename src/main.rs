
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: file_reader.exe <filename>");
        process::exit(1);
    }

    let filename = &args[1];

    match fs::read_to_string(filename) {
        Ok(content) => {
            println!("--- Content of {} ---\n", filename);
            println!("{}", content);
            println!("\n------------------------");
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    }
}
