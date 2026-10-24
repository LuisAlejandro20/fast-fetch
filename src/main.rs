use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "fast_fetch")]
#[command(about = "A fast file searcher written in Rust")]
struct Args {
    /// The text string to look for
    query: String,

    /// The directory to search in (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    println!("Searching for '{}' in '{}'...", args.query, args.path.display());
}