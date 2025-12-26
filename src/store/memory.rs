use std::collections::HashMap;

use crate::{resp::RespValue, store::value::{Store, StoreError}};

impl Default for Store{
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn set(&mut self, key: &RespValue, value: &RespValue) -> Result<RespValue, StoreError> {
        self.map.insert(key.clone(), value.clone());
        Ok(RespValue::SimpleString(b"OK".to_vec()))
    }

    pub fn get(&self, key: &RespValue) -> Result<RespValue, StoreError> {
        match self.map.get(key) {
            Some(n) => Ok(n.clone()),
            None => Ok(RespValue::BulkString(None))
        }

    }
}
