use super::ClientState;
use crate::Cmd;

#[derive(Debug)]
pub struct Request {
    pub command:   Cmd,
    pub arguments: Vec<String>,
    pub status:    RequestStatus,
    pub finished:  bool,
    pub state:     ClientState,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum RequestStatus {
    #[default]
    Pending,
    Valid,
    Invalid,
}

impl From<&String> for Request {
    fn from(s: &String) -> Self { Request::from(s.as_str()) }
}

impl From<&str> for Request {
    fn from(value: &str) -> Self {
        let v: Vec<&str> = value.split_whitespace().collect();
        Request {
            command: Cmd::parse(v.first().unwrap()).unwrap(),
            arguments: v[1..].iter().map(|x| x.to_string()).collect(),
            ..Default::default()
        }
    }
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

    pub fn validate(&mut self) -> bool {
        match &self.command {
            Cmd::Other(_) => match &self.state {
                ClientState::Unattached => self.status = RequestStatus::Valid,
                ClientState::Attached(_) => self.status = RequestStatus::Invalid,
            },
            _ => self.status = RequestStatus::Valid,
        }
        self.status.into()
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
            command:   Cmd::Other(String::new()),
            arguments: Vec::new(),
            status:    RequestStatus::Pending,
            finished:  false,
            state:     ClientState::default(),
        }
    }
}
