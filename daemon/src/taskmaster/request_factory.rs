use std::collections::{HashMap, VecDeque};

use common::{Key, Request};
use logger::debug;

#[derive(Default)]
pub struct RequestFactory {
    pub clients: HashMap<Key, VecDeque<String>>,
}

impl RequestFactory {
    pub fn new() -> RequestFactory { Self::default() }

    pub fn insert(&mut self, k: Key, buf: &mut str) {
        if let Some(requests) = self.clients.get_mut(&k) {
            let value = requests.front_mut().unwrap();
            debug!("val: {value}\nbuf: {buf}");
            value.push_str(buf);
        } else {
            let mut vec = VecDeque::new();
            vec.push_back(buf.into());
            self.clients.insert(k, vec);
        }
    }

    pub fn parse(&mut self, k: Key) -> Option<Request> {
        let vec = self.clients.get_mut(&k).unwrap();
        debug!("Factory:  current request - {vec:?}");
        if vec.front().unwrap().contains('\n') {
            debug!("Factory: request is complete");
            let raw = vec.pop_front().unwrap();
            if vec.is_empty() {
                self.clients.remove(&k);
            }
            let mut request = Request::from(&raw);
            request.client_key = k;
            return Some(request);
        }
        None
    }

    // pub fn create_or_append(&mut self, k: key, buf: String) {}
}
