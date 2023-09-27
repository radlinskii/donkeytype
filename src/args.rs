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

    /// path to dictionary file
    #[arg(long)]
    pub dictionary_path: Option<String>,
}
