use bakme::create_copy;
use std::{fs, path::PathBuf};
use tempfile::TempDir;

const BAK_EXT: &str = "bak";
const TEST_PATHS: &[&[&str]] = &[
    &["copy-test", "subdir", "lvl3"],
    &["unicode", "über", "фолдер"],
];

fn setup_test_structure(base: &TempDir) -> PathBuf {
    let src: std::path::PathBuf = base.path().join("source");

    TEST_PATHS.iter().for_each(|folders| {
        fs::create_dir_all(src.join(folders.join("/"))).unwrap();

        let mut sub_path = src.clone();
        folders.iter().for_each(|sub_folder| {
            sub_path = sub_path.join(sub_folder);
            fs::write(sub_path.join("file1.txt"), "hello").unwrap();
            fs::write(sub_path.join("fіlе2.txt"), "world").unwrap();
        });
    });

    src
}

#[test]
fn test_copy_single_file() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("test.txt");
    fs::write(&src, "test content").unwrap();

    let mut dest = src.clone();
    dest.add_extension(BAK_EXT);
    if let Ok(bak_path) = create_copy(src.to_str().unwrap(), "bak", false, false) {
        assert!(
            fs::exists(&bak_path).unwrap_or_default(),
            "Backup file should be copied"
        );
        assert!(dest.exists(), "Backup file should exist");
        assert_eq!(&bak_path, &dest, "Report and expect backup path differs");

        assert_eq!(
            std::fs::read(src).unwrap_or_default(),
            std::fs::read(dest).unwrap_or_default(),
            "Original and backup content differs"
        );
    } else {
        panic!("Copy function returned an error")
    }
}

#[test]
fn test_copy_directory_with_files() {
    let tmp = TempDir::new().unwrap();
    let src = setup_test_structure(&tmp);

    let mut dest = src.clone();
    dest.add_extension(BAK_EXT);
    if let Ok(bak_path) = create_copy(src.to_str().unwrap(), "bak", false, false) {
        assert!(
            fs::exists(&bak_path).unwrap_or_default(),
            "Backup folder should be copied"
        );
        assert!(dest.exists(), "Backup folder should exist");
        assert_eq!(&bak_path, &dest, "Report and expect backup path differs");

        assert_eq!(
            std::fs::read(src).unwrap_or_default(),
            std::fs::read(dest).unwrap_or_default(),
            "Original and backup content differs"
        );
    } else {
        panic!("Copy function returned an error")
    }

    // TODO: add an actual folder/files comparison
}
