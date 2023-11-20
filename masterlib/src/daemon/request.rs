pub struct Request {
    pub command: String,
    pub arguments: Vec<String>,
    pub valid: bool,
    pub finished: bool,
}

impl Request {
    pub fn new() -> Request {
        Self::default()
    }
}
impl Default for Request {
    fn default() -> Self {
        Request {
            command: String::default(),
            arguments: Vec::new(),
            valid: true,
            finished: false,
        }
    }
}
