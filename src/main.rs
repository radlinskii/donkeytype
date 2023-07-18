mod cli;

use clap::Parser;
use cli::print::print_hello;
use dirs;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    print_hello();

    let args = Args::parse();

    let home_dir = dirs::home_dir().unwrap();
    let donkey_type_config_dir = home_dir
        .join(".config")
        .join("donkey-type")
        .join("donkey-type.lua");

    if !donkey_type_config_dir.exists() {
        println!("Donkey Type config directory does not exist");
    } else {
        println!("Donkey Type config directory exists");
    }

    for _ in 0..args.count {
        println!("Hello {}!", args.name);

        println!("Your home directory is: {}", home_dir.display());
    }

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(donkey_type_config_dir) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
