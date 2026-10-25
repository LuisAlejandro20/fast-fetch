use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;
use rayon::prelude::*;
use colored::*;

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
            for (index, line) in content.lines().enumerate() {
                if line.contains(&args.query) {
                    let highlighted = line.replace(
                        &args.query,
                        &args.query.red().bold().to_string()
                    );

                    println!("
                {}:{} {}",
                path.display().to_string().cyan(),
                (index + 1).to_string().yellow(),
                highlighted)
                };
            }
        }
    })
}