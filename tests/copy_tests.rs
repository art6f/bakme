use bakme::create_copy;
use std::fs;
use tempfile::TempDir;

const BAK_EXT: &str = "bak";
const TEST_PATHS: &[&[&str]] = &[
    &["copy-test", "subdir", "lvl3"],
    &["unicode", "über", "фолдер"],
];

fn setup_test_structure(base: &TempDir) -> String {
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

    src.to_string_lossy().to_string()
}

#[test]
fn test_copy_single_file() {
    let tmp = TempDir::new().unwrap();
    let src = tmp.path().join("test.txt");
    fs::write(&src, "test content").unwrap();

    let dest = tmp.path().join("test.txt.bak");
    create_copy(src.to_str().unwrap(), "bak", false, false);
    assert!(dest.exists(), "Backup file should exist");
    assert_eq!(fs::read_to_string(&dest).unwrap(), "file content");
}

#[test]
fn test_copy_directory_with_files() {
    let tmp = TempDir::new().unwrap();
    let src = setup_test_structure(&tmp);
    let dest = format!("{src}.{BAK_EXT}");

    create_copy(&src, BAK_EXT, false, false);
    assert!(
        dest.ends_with(".bak"),
        "Destination should have .bak suffix"
    );
    assert!(fs::exists(&dest).unwrap());
    assert!(fs::read_dir(&dest).unwrap().count() >= 2);
}
//
//#[test]
//fn test_copy_file_preserves_content() {
//    let tmp = TempDir::new().unwrap();
//    let content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
//    let src = tmp.path().join("document.txt");
//    fs::write(&src, content).unwrap();
//    let dest = tmp.path().join("document.txt.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert_eq!(fs::read_to_string(&dest).unwrap(), content);
//}
//
//#[test]
//fn test_copy_file_with_custom_extension() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("data.csv");
//    fs::write(&src, "a,b,c").unwrap();
//    let dest = tmp.path().join("data.csv.backup");
//    create_copy(src.to_str().unwrap(), "backup", false, false);
//    assert!(dest.exists());
//    assert_eq!(fs::read_to_string(&dest).unwrap(), "a,b,c");
//}
//
//#[test]
//fn test_copy_file_nonexistent_path() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("does_not_exist.txt");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(!src.exists());
//}
//
//#[test]
//fn test_copy_file_empty_content() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("empty.txt");
//    fs::write(&src, "").unwrap();
//    let dest = tmp.path().join("empty.txt.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists());
//    assert_eq!(fs::read_to_string(&dest).unwrap(), "");
//}
//
//#[test]
//fn test_copy_file_binary_content() {
//    let tmp = TempDir::new().unwrap();
//    let content: Vec<u8> = vec![0u8, 1, 2, 255, 254, 0, 128];
//    let src = tmp.path().join("binary.bin");
//    fs::write(&src, &content).unwrap();
//    let dest = tmp.path().join("binary.bin.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists());
//    assert_eq!(fs::read(&dest).unwrap(), content);
//}

//#[test]
//fn test_copy_empty_directory() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("empty_dir");
//    fs::create_dir_all(&src).unwrap();
//    let dest = tmp.path().join("empty_dir.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists(), "Backup directory should exist");
//    assert!(dest.is_dir(), "Backup should be a directory");
//    assert_eq!(
//        fs::read_dir(&dest).unwrap().count(),
//        0,
//        "Directory should be empty"
//    );
//}
//
//#[test]
//fn test_copy_directory_preserves_structure() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("source");
//    fs::create_dir_all(src.join("a/b/c")).unwrap();
//    fs::write(src.join("a/b/c/deep.txt"), "deep content").unwrap();
//    fs::write(src.join("root.txt"), "root").unwrap();
//    let dest = format!("{}.bak", src.to_str().unwrap());
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let expected_deep = format!("{dest}/a/b/c/deep.txt");
//    let expected_root = format!("{dest}/root.txt");
//    assert!(
//        fs::exists(&expected_deep).unwrap(),
//        "Deep nested file should be copied"
//    );
//    assert_eq!(fs::read_to_string(&expected_deep).unwrap(), "deep content");
//    assert!(
//        fs::exists(&expected_root).unwrap(),
//        "Root file should be copied"
//    );
//    assert_eq!(fs::read_to_string(&expected_root).unwrap(), "root");
//}
//
//#[test]
//fn test_copy_directory_preserves_all_files() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("source");
//    fs::create_dir_all(src.join("subdir")).unwrap();
//    fs::write(src.join("file1.txt"), "content1").unwrap();
//    fs::write(src.join("file2.txt"), "content2").unwrap();
//    fs::write(src.join("subdir/file3.txt"), "content3").unwrap();
//    let dest = format!("{}.bak", src.to_str().unwrap());
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/file1.txt")).unwrap(),
//        "content1"
//    );
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/file2.txt")).unwrap(),
//        "content2"
//    );
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/subdir/file3.txt")).unwrap(),
//        "content3"
//    );
//}
//
//#[test]
//fn test_copy_deeply_nested_directory() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("source");
//    let mut current = src.clone();
//    for i in 0..10 {
//        current = current.join(format!("level{}", i));
//    }
//    fs::create_dir_all(&current).unwrap();
//    fs::write(current.join("leaf.txt"), "leaf").unwrap();
//    let dest = format!("{}.bak", src.to_str().unwrap());
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let expected = format!(
//        "{dest}/level0/level1/level2/level3/level4/level5/level6/level7/level8/level9/leaf.txt"
//    );
//    assert!(
//        fs::exists(&expected).unwrap(),
//        "Deeply nested file should be copied"
//    );
//    assert_eq!(fs::read_to_string(&expected).unwrap(), "leaf");
//}
//
//#[test]
//fn test_copy_file_increment_counter_on_conflict() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("file.txt");
//    fs::write(&src, "original").unwrap();
//    let backup = tmp.path().join("file.txt.bak");
//    fs::write(&backup, "old backup").unwrap();
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let expected = tmp.path().join("file.txt.bak-1");
//    assert!(expected.exists(), "Incremented backup should be created");
//    assert_eq!(fs::read_to_string(&expected).unwrap(), "original");
//}
//
//#[test]
//fn test_copy_file_multiple_counters() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("doc.txt");
//    fs::write(&src, "doc").unwrap();
//    fs::write(tmp.path().join("doc.txt.bak"), "bak0").unwrap();
//    fs::write(tmp.path().join("doc.txt.bak-1"), "bak1").unwrap();
//    fs::write(tmp.path().join("doc.txt.bak-2"), "bak2").unwrap();
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let expected = tmp.path().join("doc.txt.bak-3");
//    assert!(
//        expected.exists(),
//        "Counter should continue from existing backups"
//    );
//    assert_eq!(fs::read_to_string(&expected).unwrap(), "doc");
//}
//
//#[test]
//fn test_copy_file_respects_limit_without_force() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("file.txt");
//    fs::write(&src, "data").unwrap();
//    for i in 0..15 {
//        let ext = if i == 0 {
//            "bak".to_string()
//        } else {
//            format!("bak-{}", i - 1)
//        };
//        fs::write(
//            tmp.path().join(format!("file.txt.{}", ext)),
//            format!("backup-{}", i),
//        )
//        .unwrap();
//    }
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let limit_path = tmp.path().join("file.txt.bak-14");
//    assert!(
//        !limit_path.exists(),
//        "Should refuse to create backup beyond limit"
//    );
//}
//
//#[test]
//fn test_copy_file_force_bypasses_limit() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("file.txt");
//    fs::write(&src, "data").unwrap();
//    for i in 0..15 {
//        let ext = if i == 0 {
//            "bak".to_string()
//        } else {
//            format!("bak-{}", i - 1)
//        };
//        fs::write(
//            tmp.path().join(format!("file.txt.{}", ext)),
//            format!("backup-{}", i),
//        )
//        .unwrap();
//    }
//    create_copy(src.to_str().unwrap(), "bak", false, true);
//    let forced_path = tmp.path().join("file.txt.bak-14");
//    assert!(
//        forced_path.exists(),
//        "Force should allow backup beyond limit"
//    );
//    assert_eq!(fs::read_to_string(&forced_path).unwrap(), "data");
//}
//
//#[test]
//fn test_copy_directory_increment_counter_on_conflict() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("mydir");
//    fs::create_dir_all(&src).unwrap();
//    fs::write(src.join("a.txt"), "a").unwrap();
//    let existing = tmp.path().join("mydir.bak");
//    fs::create_dir_all(&existing).unwrap();
//    fs::write(existing.join("old.txt"), "old").unwrap();
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let expected = tmp.path().join("mydir.bak-1");
//    assert!(
//        expected.exists(),
//        "Incremented dir backup should be created"
//    );
//    assert!(expected.is_dir());
//    assert!(
//        expected.join("a.txt").exists(),
//        "Files should be copied to incremented backup"
//    );
//    assert_eq!(fs::read_to_string(expected.join("a.txt")).unwrap(), "a");
//}
//
//#[test]
//fn test_copy_file_with_timed_extension() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("report.txt");
//    fs::write(&src, "report content").unwrap();
//    create_copy(src.to_str().unwrap(), "bak", true, false);
//    let entries: Vec<_> = fs::read_dir(tmp.path())
//        .unwrap()
//        .filter_map(|e| e.ok())
//        .filter(|e| {
//            e.path()
//                .file_name()
//                .is_some_and(|n| n.to_string_lossy().starts_with("report.txt.bak_"))
//        })
//        .collect();
//    assert!(!entries.is_empty(), "Timed backup should be created");
//    assert_eq!(
//        fs::read_to_string(&entries[0].path()).unwrap(),
//        "report content"
//    );
//}
//
//#[test]
//fn test_copy_file_timed_no_conflict() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("data.csv");
//    fs::write(&src, "1,2,3").unwrap();
//    create_copy(src.to_str().unwrap(), "bak", true, false);
//    let entries: Vec<_> = fs::read_dir(tmp.path())
//        .unwrap()
//        .filter_map(|e| e.ok())
//        .filter(|e| {
//            e.path()
//                .file_name()
//                .is_some_and(|n| n.to_string_lossy().starts_with("data.csv.bak_"))
//        })
//        .collect();
//    assert_eq!(entries.len(), 1, "Should create exactly one timed backup");
//}
//
//#[test]
//fn test_copy_file_with_spaces_in_name() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("my document.txt");
//    fs::write(&src, "spaces content").unwrap();
//    let dest = tmp.path().join("my document.txt.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists());
//    assert_eq!(fs::read_to_string(&dest).unwrap(), "spaces content");
//}
//
//#[test]
//fn test_copy_file_with_unicode_name() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("文件.txt");
//    fs::write(&src, "unicode content").unwrap();
//    let dest = tmp.path().join("文件.txt.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists());
//    assert_eq!(fs::read_to_string(&dest).unwrap(), "unicode content");
//}
//
//#[test]
//fn test_copy_directory_with_unicode_names() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("数据");
//    fs::create_dir_all(&src).unwrap();
//    fs::write(src.join("报告.txt"), "报告内容").unwrap();
//    let dest = format!("{}.bak", src.to_str().unwrap());
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(fs::exists(&dest).unwrap());
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/报告.txt")).unwrap(),
//        "报告内容"
//    );
//}
//
//#[test]
//fn test_copy_file_trailing_slash_on_path() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("test.txt");
//    fs::write(&src, "content").unwrap();
//    let dest = tmp.path().join("test.txt.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists());
//}
//
//#[test]
//fn test_copy_nonexistent_directory() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("nonexistent_dir");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(!src.exists());
//}
//
//#[test]
//fn test_copy_directory_with_mixed_content() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("mixed");
//    fs::create_dir_all(src.join("sub1/sub2")).unwrap();
//    fs::create_dir_all(src.join("sub3")).unwrap();
//    fs::write(src.join("root.txt"), "root").unwrap();
//    fs::write(src.join("sub1/a.txt"), "a").unwrap();
//    fs::write(src.join("sub1/sub2/b.txt"), "b").unwrap();
//    fs::write(src.join("sub3/c.txt"), "c").unwrap();
//    let dest = format!("{}.bak", src.to_str().unwrap());
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/root.txt")).unwrap(),
//        "root"
//    );
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/sub1/a.txt")).unwrap(),
//        "a"
//    );
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/sub1/sub2/b.txt")).unwrap(),
//        "b"
//    );
//    assert_eq!(
//        fs::read_to_string(format!("{dest}/sub3/c.txt")).unwrap(),
//        "c"
//    );
//}
//
//#[test]
//fn test_copy_preserves_large_file_content() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("large.bin");
//    let content: Vec<u8> = (0..1024).flat_map(|i| vec![(i % 256) as u8]).collect();
//    fs::write(&src, &content).unwrap();
//    let dest = tmp.path().join("large.bin.bak");
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert!(dest.exists());
//    let backed = fs::read(&dest).unwrap();
//    assert_eq!(backed.len(), content.len());
//    assert_eq!(&backed, &content);
//}
//
//#[test]
//fn test_copy_directory_counter_chain() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("project");
//    fs::create_dir_all(&src).unwrap();
//    fs::write(src.join("main.rs"), "fn main() {}").unwrap();
//    for suffix in [".bak", ".bak-1", ".bak-2"] {
//        let p = tmp.path().join(format!("project{suffix}"));
//        fs::create_dir_all(&p).unwrap();
//        fs::write(p.join("old.rs"), "old").unwrap();
//    }
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    let expected = tmp.path().join("project.bak-3");
//    assert!(expected.exists());
//    assert_eq!(
//        fs::read_to_string(expected.join("main.rs")).unwrap(),
//        "fn main() {}"
//    );
//}
//
//#[test]
//fn test_copy_file_does_not_modify_original() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("original.txt");
//    let content = "do not modify me";
//    fs::write(&src, content).unwrap();
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert_eq!(fs::read_to_string(&src).unwrap(), content);
//}
//
//#[test]
//fn test_copy_directory_does_not_modify_original() {
//    let tmp = TempDir::new().unwrap();
//    let src = tmp.path().join("orig_dir");
//    fs::create_dir_all(&src).unwrap();
//    fs::write(src.join("keep.txt"), "unchanged").unwrap();
//    create_copy(src.to_str().unwrap(), "bak", false, false);
//    assert_eq!(
//        fs::read_to_string(src.join("keep.txt")).unwrap(),
//        "unchanged"
//    );
//}
//
////#[test]
////fn test_copy_dir_recursive_structure() {
////    let tmp = TempDir::new().unwrap();
////    let src = tmp.path().join("src");
////    fs::create_dir_all(src.join("a/b/c")).unwrap();
////    fs::write(src.join("a/b/c/file.txt"), "nested").unwrap();
////    let dest = tmp.path().join("dst");
////    copy_dir(src.to_str().unwrap(), dest.to_string_lossy().to_string());
////    assert!(dest.exists());
////    assert!(dest.is_dir());
////    assert!(dest.join("a/b/c/file.txt").exists());
////    assert_eq!(
////        fs::read_to_string(dest.join("a/b/c/file.txt")).unwrap(),
////        "nested"
////    );
////}
////
////#[test]
////fn test_copy_dir_multiple_subdirectories() {
////    let tmp = TempDir::new().unwrap();
////    let src = tmp.path().join("src");
////    fs::create_dir_all(src.join("dir1")).unwrap();
////    fs::create_dir_all(src.join("dir2")).unwrap();
////    fs::write(src.join("dir1/f1.txt"), "f1").unwrap();
////    fs::write(src.join("dir2/f2.txt"), "f2").unwrap();
////    fs::write(src.join("root.txt"), "root").unwrap();
////    let dest = tmp.path().join("dst");
////    copy_dir(src.to_str().unwrap(), dest.to_string_lossy().to_string());
////    assert_eq!(fs::read_to_string(dest.join("root.txt")).unwrap(), "root");
////    assert_eq!(fs::read_to_string(dest.join("dir1/f1.txt")).unwrap(), "f1");
////    assert_eq!(fs::read_to_string(dest.join("dir2/f2.txt")).unwrap(), "f2");
////}
////
////#[test]
////fn test_copy_dir_does_not_affect_source() {
////    let tmp = TempDir::new().unwrap();
////    let src = tmp.path().join("src");
////    fs::create_dir_all(&src).unwrap();
////    fs::write(src.join("x.txt"), "x").unwrap();
////    let original_len = fs::metadata(&src.join("x.txt")).unwrap().len();
////    let dest = tmp.path().join("dst");
////    copy_dir(src.to_str().unwrap(), dest.to_string_lossy().to_string());
////    assert_eq!(fs::metadata(src.join("x.txt")).unwrap().len(), original_len);
////    assert_eq!(fs::read_to_string(src.join("x.txt")).unwrap(), "x");
////}
