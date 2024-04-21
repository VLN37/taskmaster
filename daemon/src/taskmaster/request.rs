use crate::backend::DaemonCommand;

#[derive(Debug)]
pub struct Request {
    pub command:   String,
    pub arguments: Vec<String>,
    pub status:    RequestStatus,
    pub finished:  bool,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum RequestStatus {
    #[default]
    Pending,
    Valid,
    Invalid,
}

impl From<RequestStatus> for bool {
    fn from(value: RequestStatus) -> bool { value == RequestStatus::Valid }
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
        match DaemonCommand::parse(&self.command) {
            Ok(_) => self.status = RequestStatus::Valid,
            Err(_) => self.status = RequestStatus::Invalid,
        }
        self.status.into()
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
