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
            let parts: Vec<&str> = line.trim().split('|').collect();

            match parts.as_slice() {
                ["PUT", key, value] => {
                    store.insert(key.to_string(), value.to_string());
                }
                ["DELETE", key] => {
                    store.remove(*key);
                }
                _ => {
                    eprintln!("WAL: Unrecognized line format: {}", line);
                }
            }
        }

        Ok(store)
    }
}
