use std::collections::HashMap;

mod parse;

pub const HOST: &str = "127.0.0.1:6173";

pub struct Db {
    records: HashMap<String, String>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
            let data = self.records.get(key);

            if let Some(value) = data {
                return Some(value.clone());
            }
            None
    }
    

    pub async fn set(&mut self, key: String, value: String) -> Option<()> {
        self.records.insert(key, value);
        Some(())
    }

    pub async fn delete(&mut self, key: &str) -> Option<String> {
        self.records.remove(key)
    }
}
