use bakme::{VERBOSE, create_copy, verbose_print};
use clap::Parser;
use std::sync::atomic::Ordering;

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

fn main() {
    let argv = Args::parse();
    let base_ext = argv.extension.unwrap_or("bak".to_string());

    VERBOSE.store(argv.verbose, Ordering::Relaxed);
    verbose_print(format!("Backing up {} objects(s)...", argv.files.len()));

    argv.files.iter().for_each(|path| {
        let _ = create_copy(
            path.trim_end_matches(['/', '\\']),
            &base_ext,
            argv.timed,
            argv.force,
        );
    });
}
