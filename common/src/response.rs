use crate::Key;

#[derive(Debug)]
pub struct Response {
    pub message:    String,
    pub finished:   bool,
    pub client_key: Key,
}
