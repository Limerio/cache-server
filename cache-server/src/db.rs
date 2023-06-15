use bytes::Bytes;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Db {
    pub data: Arc<Mutex<HashMap<String, Bytes>>>,
}

#[derive(Serialize)]
struct KeyValue {
    key: String,
    value: String,
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

        match map.get(&k) {
            Some(value) => String::from_utf8_lossy(value).to_string(),
            None => format!("Value not found for key: {}", k),
        }
    }

    pub fn del(&mut self, k: String) {
        let mut map = self.data.lock().unwrap();

        map.remove(&k);
    }

    pub fn all(&mut self) -> String {
        let map = self.data.lock().unwrap();

        let key_values: Vec<KeyValue> = map
            .iter()
            .map(|(key, value)| KeyValue {
                key: key.clone(),
                value: String::from_utf8_lossy(&value.clone()).to_string(),
            })
            .collect();

        serde_json::to_string(&key_values).unwrap()
    }
}
