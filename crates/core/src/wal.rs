use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write, Seek};
use std::path::Path;
use std::collections::HashMap;
use std::io;

pub struct Wal {
    log_file: File,
}

impl Wal {
    /// Opens or creates the WAL file at the given path.
    pub fn open(path: &Path) -> io::Result<Wal> {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
        Ok(Wal { log_file })
    }

    /// Appends a PUT command to the WAL.
    pub fn append_put(&mut self, key: &str, value: &str) -> io::Result<()> {
        writeln!(self.log_file, "PUT|{}|{}", key, value)
    }

    /// Appends a DELETE command to the WAL.
    pub fn append_delete(&mut self, key: &str) -> io::Result<()> {
        writeln!(self.log_file, "DELETE|{}", key)
    }

    /// Loads all log entries into a HashMap as the current store state.
    pub fn load_into(&mut self) -> io::Result<HashMap<String, String>> {
        self.log_file.rewind()?;
        let reader = BufReader::new(&self.log_file);

        let mut store = HashMap::new();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.split('|').collect();

            if parts.len() == 3 && parts[0] == "PUT" {
                store.insert(parts[1].to_string(), parts[2].to_string());
            } else if parts.len() == 2 && parts[0] == "DELETE" {
                store.remove(parts[1]);
            } else {
                eprintln!("WAL: Unrecognized line format: {}", line);
                // Do not touch the map
            }
        }

        Ok(store)
    }

    /// Truncate the WAL file (clear all contents).
    pub fn truncate(&mut self, path: &Path) -> io::Result<()> {
        // Re-open in write mode to truncate
        self.log_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(())
    }
}

// Example usage in your codebase
pub const WAL_PATH: &str = ".zyncdb.wal";
pub const SNAPSHOT_PATH: &str = ".zyncdb.snapshot";
