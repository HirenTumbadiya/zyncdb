use crate::wal::Wal;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use storage::{FileStorage, MemStorage, Storage};

pub struct KvStore {
    storage: Box<dyn Storage>,
    wal: Option<Arc<Mutex<Wal>>>,
    expirations: HashMap<String, Instant>,
    tx_buffer: Option<HashMap<String, String>>,
}

pub enum Backend {
    Memory,
    File(String),
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
            expirations: HashMap::new(),
            tx_buffer: None,
        })
    }

    pub fn open_with_backend(path: &Path, backend: Backend) -> std::io::Result<Self> {
        let mut wal = Wal::open(path)?;
        let map = wal.load_into()?;

        let storage: Box<dyn Storage> = match backend {
            Backend::Memory => {
                let mut mem = MemStorage::new();
                for (k, v) in map {
                    mem.insert(k, v);
                }
                Box::new(mem)
            }
            Backend::File(file_path) => {
                let mut file_storage = FileStorage::new(file_path)?;
                for (k, v) in map {
                    file_storage.insert(k, v);
                }
                Box::new(file_storage)
            }
        };

        Ok(KvStore {
            storage,
            wal: Some(Arc::new(Mutex::new(wal))),
            expirations: HashMap::new(),
            tx_buffer: None,
        })
    }

    /// Create a snapshot of the current state and truncate the WAL.
    /// This will write the current key-value pairs to a snapshot file and clear the WAL.
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
            expirations: HashMap::new(),
            tx_buffer: None,
        })
    }

    pub fn set_ttl(&mut self, key: &str, ttl_secs: u64) {
        self.expirations
            .insert(key.to_string(), Instant::now() + Duration::from_secs(ttl_secs));
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        // Check expiration
        if let Some(expiry) = self.expirations.get(key) {
            if Instant::now() > *expiry {
                self.storage.delete(key);
                self.expirations.remove(key);
                return None;
            }
        }
        self.storage.get(key)
    }

    pub fn len(&self) -> usize {
        self.storage.len()
    }

    pub fn clear(&mut self) {
        self.storage.clear();
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        if let Some(buf) = &mut self.tx_buffer {
            buf.insert(key, value)
        } else {
            if let Some(wal) = &self.wal {
                if let Ok(mut wal) = wal.lock() {
                    if let Err(e) = wal.append_put(&key, &value) {
                        eprintln!("WAL append_put error: {}", e);
                    }
                }
            }
            self.storage.insert(key, value)
        }
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if self.storage.get(key).is_some() {
            if let Some(buf) = &mut self.tx_buffer {
                buf.remove(key);
                true
            } else {
                if let Some(wal) = &self.wal {
                    if let Ok(mut wal) = wal.lock() {
                        if let Err(e) = wal.append_delete(key) {
                            eprintln!("WAL append_delete error: {}", e);
                        }
                    }
                }
                self.storage.delete(key)
            }
        } else {
            false
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.storage.get(key).is_some()
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = (String, String)> + '_> {
        self.storage.iter()
    }

    pub fn begin_tx(&mut self) {
        self.tx_buffer = Some(HashMap::new());
    }
    pub fn commit_tx(&mut self) {
        if let Some(buf) = self.tx_buffer.take() {
            for (k, v) in buf {
                self.storage.insert(k, v);
            }
        }
    }
    pub fn rollback_tx(&mut self) {
        self.tx_buffer = None;
    }
}
