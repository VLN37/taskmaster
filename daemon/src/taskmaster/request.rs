#[derive(Debug)]
pub struct Request {
    pub command:   String,
    pub arguments: Vec<String>,
    pub status:    RequestStatus,
    pub finished:  bool,
}

#[derive(Default, Debug, PartialEq)]
pub enum RequestStatus {
    #[default]
    Pending,
    Valid,
    Invalid,
}

impl Request {
    pub fn new() -> Request { Self::default() }

    pub fn is_valid(&mut self) -> bool {
        match self.status {
            RequestStatus::Valid => true,
            RequestStatus::Invalid => false,
            RequestStatus::Pending => self.validate(),
        }
    }

    fn validate(&mut self) -> bool {
        match self.command.to_uppercase().as_str() {
            "ATTACH" => self.status = RequestStatus::Valid,
            "STATUS" => self.status = RequestStatus::Valid,
            "LOG" => self.status = RequestStatus::Valid,
            "HEAD" => self.status = RequestStatus::Valid,
            _ => self.status = RequestStatus::Invalid,
        };
        self.status == RequestStatus::Valid
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
            command:   String::default(),
            arguments: Vec::new(),
            status:    RequestStatus::Pending,
            finished:  false,
        }
    }
}
