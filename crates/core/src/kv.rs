use crate::wal::Wal;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use storage::{MemStorage, Storage};

pub struct KvStore {
    storage: Box<dyn Storage>,
    wal: Option<Arc<Mutex<Wal>>>,
}

impl KvStore {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let mut wal = Wal::open(path)?;
        let map = wal.load_into()?;

        // Initialize MemStorage with data from WAL
        let mut mem = MemStorage::new();
        for (k, v) in map {
            mem.insert(k, v);
        }

        Ok(KvStore {
            storage: Box::new(mem),
            wal: Some(Arc::new(Mutex::new(wal))),
        })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.storage.get(key)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        if let Some(wal) = &self.wal {
            if let Ok(mut wal) = wal.lock() {
                if let Err(e) = wal.append_put(&key, &value) {
                    eprintln!("WAL append_put error: {}", e);
                }
            }
        }
        self.storage.insert(key, value)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if self.storage.get(key).is_some() {
            if let Some(wal) = &self.wal {
                if let Ok(mut wal) = wal.lock() {
                    if let Err(e) = wal.append_delete(key) {
                        eprintln!("WAL append_delete error: {}", e);
                    }
                }
            }
            self.storage.delete(key)
        } else {
            false
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.storage.get(key).is_some()
    }

    /// Write the current state to a snapshot file and truncate the WAL.
    pub fn snapshot_and_compact(
        &mut self,
        snapshot_path: &Path,
        wal_path: &Path,
    ) -> io::Result<()> {
        // 1. Write snapshot
        let file = File::create(snapshot_path)?;
        let mut writer = BufWriter::new(file);
        for (k, v) in self.storage.iter() {
            writeln!(writer, "{}|{}", k, v)?;
        }
        writer.flush()?;

        // 2. Truncate WAL
        if let Some(wal) = &self.wal {
            let mut wal = wal.lock().unwrap();
            wal.truncate(wal_path)?;
        }
        Ok(())
    }

    /// Load from snapshot, then replay WAL.
    pub fn open_with_snapshot(snapshot_path: &Path, wal_path: &Path) -> io::Result<Self> {
        let mut mem = MemStorage::new();

        // 1. Load snapshot if exists
        if snapshot_path.exists() {
            let file = File::open(snapshot_path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let parts: Vec<&str> = line.trim().split('|').collect();
                if parts.len() == 2 {
                    mem.insert(parts[0].to_string(), parts[1].to_string());
                }
            }
        }

        // 2. Replay WAL
        let mut wal = Wal::open(wal_path)?;
        let wal_map = wal.load_into()?;
        for (k, v) in wal_map {
            mem.insert(k, v);
        }

        Ok(KvStore {
            storage: Box::new(mem),
            wal: Some(Arc::new(Mutex::new(wal))),
        })
    }
}
