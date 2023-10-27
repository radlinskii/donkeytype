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

    /// indicates if test should include symbols
    #[arg(short, long)]
    pub symbols: Option<bool>,

    /// symbols-ratio argument
    #[arg(long)]
    pub symbols_ratio: Option<f64>,

    /// path to dictionary file
    #[arg(long)]
    pub dictionary_path: Option<String>,

    /// indicates if test should include words beginning with uppercase letters
    #[arg(short, long)]
    pub uppercase: Option<bool>,

    /// uppercase-ratio argument
    #[arg(long)]
    pub uppercase_ratio: Option<f64>,

    /// indicates if test results should be saved
    #[arg(long)]
    pub save_results: Option<bool>,

    /// path to save the test results, enabled only when save_results is true
    #[arg(long, requires = "save_results")]
    pub results_path: Option<String>,

    /// Add subcommands here
    #[command(subcommand)]
    pub history: Option<SubCommand>,
}

#[derive(Parser, Debug, Clone)]
pub enum SubCommand {
    #[command(about = "Show previous test results in a bar chart.")]
    History(HistorySubcommandArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct HistorySubcommandArgs {
    // Define subcommand-specific arguments here
    // #[arg(short, long)]
    // pub show_date: Option<bool>,
}
