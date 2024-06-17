use std::collections::HashMap;

struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> KeyValueStore{
        KeyValueStore { store: HashMap::new() }
    }

    fn insert(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
}



fn main() {
    
}

// i want to build documents handler using Rust as database engine
