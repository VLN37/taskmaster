use std::collections::VecDeque;

use common::{ClientState, Request};

#[derive(Debug, Default)]
pub struct Client {
    pub state:    ClientState,
    pub requests: VecDeque<Request>,
}

impl Client {
    pub fn new() -> Client { Client::default() }
}
