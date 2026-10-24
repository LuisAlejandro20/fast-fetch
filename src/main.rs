use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

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

    for entry in WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                let path = entry.path();

                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains(&args.query) {
                        println!("Match found in {}", path.display());
                    }
                }
            }
        }
}