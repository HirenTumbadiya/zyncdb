use std::collections::HashMap;
use std::fs::{File, remove_file};
use std::io::Write;
use std::path::PathBuf;

use zyncdb_core::wal::Wal;

fn temp_path() -> PathBuf {
    let unique = format!("zyncdb_test_{}.wal", uuid::Uuid::new_v4());
    let path = std::env::temp_dir().join(unique);
    let _ = remove_file(&path);
    path
}

#[test]
fn test_append_and_load() {
    let path = temp_path();

    {
        // Write WAL
        let mut wal = Wal::open(&path).expect("Failed to open WAL");
        wal.append_put("user", "alice").unwrap();
        wal.append_put("lang", "rust").unwrap();
        wal.append_delete("user").unwrap();

        drop(wal);
    }

    {
        // Read WAL
        let mut wal = Wal::open(&path).expect("Failed to reopen WAL");
        let map = wal.load_into().expect("Failed to load WAL");

        let mut expected = HashMap::new();
        expected.insert("lang".to_string(), "rust".to_string());

        assert_eq!(map, expected);
    }

    let _ = remove_file(&path); // clean up
}

#[test]
fn test_empty_log() {
    let path = temp_path();

    {
        File::create(&path).unwrap(); // create empty log
        let mut wal = Wal::open(&path).unwrap();
        let map = wal.load_into().unwrap();
        assert!(map.is_empty());
    }

    let _ = remove_file(&path);
}

#[test]
fn test_invalid_format_lines() {
    let path = temp_path();

    {
        let mut file = File::create(&path).unwrap();
        writeln!(file, "PUT|foo|bar").unwrap();
        writeln!(file, "INVALID_LINE").unwrap(); // corrupted
        writeln!(file, "DELETE|foo").unwrap();
    }

    {
        let mut wal = Wal::open(&path).unwrap();
        let map = wal.load_into().unwrap();
        assert!(map.is_empty()); // foo added then deleted
    }

    let _ = remove_file(&path);
}
