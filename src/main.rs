use core::str;
use std::fs;

use clap::Parser;

/// Quick and easy bak-ing up the files by copying them
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Extension to appent to the file (default: bak)
    #[arg(short, long)]
    extension: Option<String>,

    /// Use the extenstion with date in format "_YYYYMMDD_HHMMSS", for e.g.: .bak_20261028_113259
    #[arg(short, long)]
    timed: bool,

    files: Vec<String>,
}

fn main() {
    let argv = Args::parse();

    let base_ext = argv.extension.unwrap_or("bak".to_string());

    argv.files
        .iter()
        .for_each(|path| create_copy(path, &base_ext, argv.timed));
}

fn create_copy(path: &String, base_ext: &str, timed: bool) {
    if !std::path::Path::new(path).try_exists().unwrap_or(false) {
        println!("Path found or invalid '{path}'");
        return;
    }

    let mut counter: u32 = 0;

    let mut extenstion: String;

    let canonical_ext: String;
    if timed {
        canonical_ext = base_ext.to_string()
            + chrono::Local::now()
                .format("_%Y%m%d_%H%M%S")
                .to_string()
                .as_str();
    } else {
        canonical_ext = base_ext.to_string();
    }

    let mut new_path = format!("{path}.{canonical_ext}");
    while std::path::Path::new(&new_path)
        .try_exists()
        .unwrap_or(false)
    {
        println!("Backup name already exists '{new_path}'");

        counter += 1;
        extenstion = format!("{canonical_ext}-{counter}");
        new_path = format!("{path}.{extenstion}");
    }

    println!("Backing up to '{new_path}'...");

    if std::path::Path::new(path).is_file() {
        if fs::copy(path, new_path).is_err() {
            println!("Error backing-up tile '{path}'");
        }
    } else {
        println!("Sorry, folder are not yet supported...")
    }
}
