use clap::Parser;

/// a very minimalistic cli typing test
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
