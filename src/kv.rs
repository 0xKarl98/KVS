use std::collections::HashMap;
use std::option::Option;

//Define a hashmap to store key-value
#[derive(Default)]

pub struct KvStore {
    map: HashMap<String, String>,
}

//Implement this hashmap
impl KvStore {
    // create the kv store
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
    // given key , get the value
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned();
    }

    // Remove the given key
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
