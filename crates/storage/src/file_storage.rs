use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write, BufRead, Result};
use std::path::Path;

use super::Storage;

pub struct FileStorage {
    map: HashMap<String, String>,
    file_path: String,
}

impl FileStorage {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let file_path_str = file_path.as_ref().to_string_lossy().to_string();
        let mut map = HashMap::new();

        // Load existing data if file exists
        if let Ok(file) = File::open(&file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.trim().splitn(2, '=').collect();
                if parts.len() == 2 {
                    map.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        Ok(Self { map, file_path: file_path_str })
    }

    fn persist(&self) -> Result<()> {
        let file = OpenOptions::new().write(true).truncate(true).create(true).open(&self.file_path)?;
        let mut writer = BufWriter::new(file);
        for (k, v) in &self.map {
            writeln!(writer, "{}={}", k, v)?;
        }
        writer.flush()
    }
}

impl Storage for FileStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
    fn insert(&mut self, key: String, value: String) -> Option<String> {
        let prev = self.map.insert(key, value);
        let _ = self.persist();
        prev
    }
    fn delete(&mut self, key: &str) -> bool {
        let existed = self.map.remove(key).is_some();
        let _ = self.persist();
        existed
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (String, String)> + '_> {
        Box::new(self.map.clone().into_iter())
    }
    fn len(&self) -> usize {
        self.map.len()
    }
    fn clear(&mut self) {
        self.map.clear();
        let _ = self.persist();
    }
}