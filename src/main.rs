use std::collections::VecDeque;
use std::fs::read_dir;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Args {
    /// The path of where to start searching
    path: PathBuf,
    /// The pattern that is searched for
    pattern: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut dirs = VecDeque::new();
    dirs.push_back(args.path);
    let mut found_entries: Vec<PathBuf> = Vec::new();

    let now = Instant::now();

    // find all directories in the path
    while let Some(dir) = dirs.pop_back() {
        match read_dir(&dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let entry_path = entry.path();
                    if entry_path.to_str().unwrap().contains(&args.pattern) {
                        found_entries.push(entry_path.clone());
                    }
                    if entry.file_type().unwrap().is_dir() {
                        dirs.push_back(entry_path);
                    }
                }
            }
            Err(e) => anyhow::bail!("Error reading directory: {}", e),
        }
    }
    println!("Found {:?} occurrences in {:?}", found_entries.len(), now.elapsed());
    found_entries.iter().for_each(|entry| { println!("{:?}", entry) });
    Ok(())
}
