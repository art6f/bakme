use core::str;
use std::{
    fs,
    sync::atomic::{AtomicBool, Ordering},
};

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

    /// Proceed backing up, disregard all the errors
    #[arg(short, long)]
    force: bool,

    /// Print some extra info during backing things up
    #[arg(long)]
    verbose: bool,

    files: Vec<String>,
}

static VERBOSE: AtomicBool = AtomicBool::new(false);
fn main() {
    let argv = Args::parse();
    VERBOSE.store(argv.verbose, Ordering::Relaxed);

    let base_ext = argv.extension.unwrap_or("bak".to_string());

    verbose_print(format!("Backing up {} file(s)...", argv.files.len()));

    argv.files
        .iter()
        .for_each(|path| create_copy(path, &base_ext, argv.timed, argv.force));
}

static BAK_LIMITS: u32 = 15;
fn create_copy(path: &String, base_ext: &str, timed: bool, force: bool) {
    if !std::path::Path::new(path).try_exists().unwrap_or(false) {
        println!("Path found or invalid '{path}'");
        return;
    }

    let mut counter: u32 = 0;

    let mut extenstion: String;

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
        extenstion = format!("{canonical_ext}-{counter}");
        new_path = format!("{path}.{extenstion}");

        if !force && counter >= BAK_LIMITS {
            println!("You have at least {BAK_LIMITS} copies of '{path}.{canonical_ext}', refusing to continue");
            println!("Use --force parameter if you want to ignore it");
            return;
        }
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

fn verbose_print(msg: String) {
    if VERBOSE.load(Ordering::Relaxed) {
        println!("{msg}")
    }
}
