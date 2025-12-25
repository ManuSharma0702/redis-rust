use std::collections::HashMap;

use crate::resp::RespValue;

pub struct Store{
    pub map: HashMap<RespValue, RespValue>
}

pub enum StoreError {
    Failed,
    NotFound
}
