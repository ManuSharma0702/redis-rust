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
        match self.map.insert(key.clone(), value.clone()) {
            Some(_) => Ok(RespValue::SimpleString(b"Ok".to_vec())),
            None => Err(StoreError::Failed)
        }
    }

    pub fn get(&self, key: &RespValue) -> Result<RespValue, StoreError> {
        match self.map.get(key) {
            Some(n) => Ok(n.clone()),
            None => Err(StoreError::NotFound)
        }

    }
}
