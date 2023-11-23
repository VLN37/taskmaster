use std::collections::HashMap;

use common::server::{Key, Request};

#[derive(Default)]
pub struct BackEnd {
    pub clients: HashMap<Key, Request>,
}

impl BackEnd {
    pub fn new() -> BackEnd { BackEnd::default() }

    pub fn get_response_for(&self, key: Key) -> String { format!("Response for {key}") }
}
