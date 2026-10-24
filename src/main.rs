use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use rayon::prelude::*;

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

    let files: Vec<PathBuf> = WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_owned())
        .collect();

    files.into_par_iter().for_each(|path| {
        if let Ok(content) = fs::read_to_string(&path) {
            if content.contains(&args.query) {
                println!("Match found in {}", path.display());
            }
        }
    })
}