use bytes::Bytes;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Db {
    pub data: Arc<Mutex<HashMap<String, Bytes>>>,
}

impl Default for Db {
    fn default() -> Self {
        Self::new()
    }
}

impl Db {
    pub fn new() -> Self {
        Self {
            data: Arc::new(Mutex::new(HashMap::<String, Bytes>::new())),
        }
    }

    pub fn set(&mut self, k: String, v: Bytes) {
        let mut map = self.data.lock().unwrap();

        map.insert(k, v);
    }

    pub fn get(&mut self, k: String) -> String {
        let map = self.data.lock().unwrap();

        println!("{:?}", map.get(&k));

        match map.get(&k) {
            Some(value) => String::from_utf8_lossy(value).to_string(),
            None => format!("Value not found for key: {}", k),
        }
    }

    pub fn clone(&mut self) -> Self {
        self.clone()
    }
}
