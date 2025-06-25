use std::collections::HashMap;

pub trait Storage {
    fn get(&self, key: &str) -> Option<String>;
    fn insert(&mut self, key: String, value: String) -> Option<String>;
    fn delete(&mut self, key: &str) -> bool;
    fn iter(&self) -> Box<dyn Iterator<Item = (String, String)> + '_>;
    fn len(&self) -> usize;
    fn clear(&mut self);
}

pub struct MemStorage {
    map: HashMap<String, String>,
}

impl MemStorage {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
}

impl Storage for MemStorage {
    fn get(&self, key: &str) -> Option<String> {
        self.map.get(key).cloned()
    }
    fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.map.insert(key, value)
    }
    fn delete(&mut self, key: &str) -> bool {
        self.map.remove(key).is_some()
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (String, String)> + '_> {
        Box::new(self.map.clone().into_iter())
    }
    fn len(&self) -> usize {
        self.map.len()
    }
    fn clear(&mut self) {
        self.map.clear();
    }
}