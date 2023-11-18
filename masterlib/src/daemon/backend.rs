use super::server::Key;
use std::collections::HashMap;

#[derive(Default)]
pub struct BackEnd {
    pub clients: HashMap<u64, Key>,
}

impl BackEnd {
    pub fn new() -> BackEnd {
        BackEnd::default()
    }
}
