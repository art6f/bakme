use std::{
    fs,
    sync::atomic::{AtomicBool, Ordering},
};

use clap::Parser;

/// Quick and easy bak-ing up the files by copying them
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Extension to append to the file (default: bak)
    #[arg(short, long)]
    extension: Option<String>,

    /// Use the extenstion with date in format "_YYYYMMDD_HHMMSS", for e.g.: .bak_20261028_113259
    #[arg(short, long)]
    timed: bool,

    /// Proceed backing up, disregard all the errors
    #[arg(short, long)]
    force: bool,

    /// Print some extra info during backing things up
    #[arg(long)]
    verbose: bool,

    files: Vec<String>,
}

static VERBOSE: AtomicBool = AtomicBool::new(false);
fn verbose_print(msg: String) {
    if VERBOSE.load(Ordering::Relaxed) {
        println!("{msg}")
    }
}

fn main() {
    let argv = Args::parse();
    let base_ext = argv.extension.unwrap_or("bak".to_string());
    
    VERBOSE.store(argv.verbose, Ordering::Relaxed);    
    verbose_print(format!("Backing up {} objects(s)...", argv.files.len()));
    
    argv.files.iter().for_each(|path| {
        create_copy(
            path.trim_end_matches(['/', '\\']),
            &base_ext,
            argv.timed,
            argv.force,
        )
    });
}

static BAK_LIMITS: u32 = 15;
fn create_copy(path: &str, base_ext: &str, timed: bool, force: bool) {
    if !std::path::Path::new(path).try_exists().unwrap_or(false) {
        println!("Path not found or invalid '{path}'");
        return;
    }

    verbose_print(format!("Targed path found '{path}'"));

    let mut counter: u32 = 0;
    let mut extension: String;

    let canonical_ext: String = if timed {
        base_ext.to_string()
            + chrono::Local::now()
                .format("_%Y%m%d_%H%M%S")
                .to_string()
                .as_str()
    } else {
        base_ext.to_string()
    };

    let mut new_path = format!("{path}.{canonical_ext}");
    while std::path::Path::new(&new_path)
        .try_exists()
        .unwrap_or(false)
    {
        verbose_print(format!("Backup name already exists '{new_path}'"));

        counter += 1;
        extension = format!("{canonical_ext}-{counter}");
        new_path = format!("{path}.{extension}");

        if !force && counter >= BAK_LIMITS {
            println!(
                "You have at least {BAK_LIMITS} copies of '{path}.{canonical_ext}', refusing to continue"
            );
            println!("Use --force parameter if you want to ignore it");
            return;
        }
    }

    if std::path::Path::new(path).is_file() {
        println!("Backing file '{path}' to '{new_path}'");

        if fs::copy(path, new_path).is_err() {
            println!("Error backing-up file '{path}'");
        }
    } else {
        println!("Backing a folder {path} to '{new_path}'");
        copy_dir(path, new_path);
    }
}

fn copy_dir(path: &str, new_path: String) {
    if fs::create_dir(&new_path).is_err() {
        println!("Error creating directory '{}'", new_path);
        return;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();

            verbose_print(format!("Processing {}", entry_path.to_string_lossy()));

            let entry_name = entry_path.file_name().unwrap_or_default().to_string_lossy();
            let new_path = format!("{new_path}/{entry_name}");

            if entry_path.is_dir() {
                copy_dir(&entry_path.to_string_lossy(), new_path);
            } else if entry_path.is_file() {
                fs::copy(&entry_path, &new_path).ok();
            }
        }
    }
}
