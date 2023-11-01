use std::collections::HashMap;

pub struct Cache<T, U> {
    map: HashMap<T, U>
}

impl <T, U> Cache<T, U> {
    pub fn new() -> Self {
        Cache { map: HashMap::new() }
    }
    pub fn set(&mut self, key: T, value: U) where T: std::cmp::Eq + std::hash::Hash  {
        self.map.insert(key, value);
    }
    pub fn get(&self, key: &T) -> Option<&U> where T: std::cmp::Eq + std::hash::Hash  {
        self.map.get(key)
    }
    pub fn delete(&mut self, key: &T) where T: std::cmp::Eq + std::hash::Hash  {
        self.map.remove(key);
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn length(&self) -> usize {
        self.map.len()
    }
}