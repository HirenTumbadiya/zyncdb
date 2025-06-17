use crate::wal::Wal;
use std::collections::HashMap;
use std::path::Path;

pub struct KvStore {
    map: HashMap<String, String>,
    wal: Option<Wal>,
}

impl KvStore {
    pub fn open(path: &Path) -> std::io::Result<Self> {
        let mut wal = Wal::open(path)?;
        let map = wal.load_into()?;

        Ok(KvStore {
            map,
            wal: Some(wal),
        })
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        if let Some(wal) = &mut self.wal {
            let _ = wal.append_put(&key, &value); // handle error as needed
        }
        self.map.insert(key, value)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        if self.map.contains_key(key) {
            if let Some(wal) = &mut self.wal {
                let _ = wal.append_delete(key); // handle error as needed
            }
            self.map.remove(key);
            true
        } else {
            false
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }
}
