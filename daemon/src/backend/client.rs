use std::collections::VecDeque;

use common::request::Request;

#[derive(Debug, Default)]
pub enum ClientState {
    #[default]
    Unattached,
    Attached(String),
}

#[derive(Debug, Default)]
pub struct Client {
    pub state:    ClientState,
    pub requests: VecDeque<Request>,
}

impl Client {
    pub fn new() -> Client { Client::default() }
}
