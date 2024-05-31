use std::collections::VecDeque;

use common::{ClientState, Request, Response};

#[derive(Debug, Default)]
pub struct Client {
    pub state:     ClientState,
    pub requests:  VecDeque<Request>,
    pub responses: VecDeque<Response>,
}

impl Client {
    pub fn new() -> Client { Client::default() }
}
