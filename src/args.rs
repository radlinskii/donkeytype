//! Module with available arguments to the program
//!
//! Using `clap` crate for parsing the arguments

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "donkeytype - a very minimalistic cli typing test", long_about = None)]
pub struct Args {
    /// duration of the test in seconds
    #[arg(short, long)]
    pub duration: Option<u64>,

    /// indicates if test should include numbers
    #[arg(short, long)]
    pub numbers: Option<bool>,
    
    /// numbers-ratio argument
    #[arg(long)]
    pub numbers_ratio: Option<f64>,

    /// path to dictionary file
    #[arg(long)]
    pub dictionary_path: Option<String>,

    /// indicates if test should include words beginning with uppercase letters
    #[arg(short, long)]
    pub uppercase: Option<bool>
}
