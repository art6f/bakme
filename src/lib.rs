use std::{
    fs,
    sync::atomic::{AtomicBool, Ordering},
};

pub static VERBOSE: AtomicBool = AtomicBool::new(false);

pub fn verbose_print(msg: String) {
    if VERBOSE.load(Ordering::Relaxed) {
        println!("{msg}")
    }
}

static BAK_LIMITS: u32 = 15;
pub fn create_copy(path: &str, base_ext: &str, timed: bool, force: bool) -> Result<String, String> {
    if !std::path::Path::new(path).try_exists().unwrap_or(false) {
        let err = "Path not found or invalid '{path}'";
        println!("{err}");
        return Err(err.into());
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
            let err = format!(
                "You have at least {BAK_LIMITS} copies of '{path}.{canonical_ext}', refusing to continue"
            );
            println!("{err}\nUse --force parameter if you want to ignore it");

            return Err(err);
        }
    }

    if std::path::Path::new(path).is_file() {
        println!("Backing file '{path}' to '{new_path}'");

        if fs::copy(path, &new_path).is_err() {
            println!("Error backing-up file '{path}'");
        }
    } else {
        println!("Backing a folder {path} to '{new_path}'");
        copy_dir(path, &new_path);
    }

    Ok(new_path)
}

fn copy_dir(path: &str, new_path: &str) {
    if fs::create_dir(new_path).is_err() {
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
                copy_dir(&entry_path.to_string_lossy(), &new_path);
            } else if entry_path.is_file() {
                fs::copy(&entry_path, &new_path).ok();
            }
        }
    }
}
