use anyhow::{bail, Context, Result};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct SharedDatabase(Arc<Mutex<Database>>);

impl SharedDatabase {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(Database::new())))
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.0.lock().unwrap().set(key, value)
    }

    pub fn get(&self, key: String) -> Result<String> {
        let db = self.0.lock().unwrap();
        let value = db.get(key)?;
        Ok(value.clone())
    }
}

pub struct Database {
    data: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.data.insert(key, value);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<&String> {
        self.data.get(&key).context("Key not found")
    }
}
