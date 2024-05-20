use anyhow::Result;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

pub struct SharedDatabase(Arc<Mutex<Database>>);

impl SharedDatabase {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Database::new())))
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    pub fn set(&mut self, key: &str, value: &str, px: Option<u64>) -> Result<()> {
        self.0.lock().unwrap().set(key, value, px)
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        self.0.lock().unwrap().get(key)
    }
}

struct Database {
    data: HashMap<String, ValueEntry>,
}

impl Database {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: &str, px: Option<u64>) -> Result<()> {
        let value = if let Some(px) = px {
            ValueEntry::new_with_exiry_date(value, Duration::from_millis(px))
        } else {
            ValueEntry::new(value)
        };

        self.data.insert(key.to_string(), value);
        Ok(())
    }

    fn get(&mut self, key: &str) -> Option<String> {
        let entry = self.data.get(key)?;

        if let Some(expiry) = entry.exiry_date {
            if expiry <= SystemTime::now() {
                self.data.remove(key);
                return None;
            }
        }

        Some(entry.value.clone())
    }
}

struct ValueEntry {
    value: String,
    exiry_date: Option<SystemTime>,
}

impl ValueEntry {
    fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            exiry_date: None,
        }
    }

    fn new_with_exiry_date(value: &str, duration: Duration) -> Self {
        Self {
            value: value.to_string(),
            exiry_date: Some(SystemTime::now() + duration),
        }
    }
}
