use super::{request::Request, server::Key};
use std::collections::HashMap;

#[derive(Default)]
pub struct BackEnd {
    pub clients: HashMap<Key, Request>,
}

impl BackEnd {
    pub fn new() -> BackEnd {
        BackEnd::default()
    }

    pub fn get_response_for(&self, key: Key) -> String {
        format!("Response for {key}")
    }
}
