use crate::server::Key;
use crate::{ClientState, Cmd, CmdErrorKind, RequestStatus};

#[derive(Debug, Clone)]
pub struct Request {
    pub command:    Cmd,
    pub arguments:  Vec<String>,
    pub status:     RequestStatus,
    pub finished:   bool,
    pub state:      ClientState,
    pub error:      Option<CmdErrorKind>,
    pub client_key: Key,
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
        self.validate_command();
        self.status == RequestStatus::Valid
    }

    fn validate_command(&mut self) {
        match &self.command {
            Cmd::Other(_) => match &self.state {
                ClientState::Attached(_) => self.status = RequestStatus::Valid,
                ClientState::Unattached => {
                    self.status = RequestStatus::Invalid;
                    self.error = Some(CmdErrorKind::NotFound(self.command.to_string()));
                }
            },
            _ => self.status = RequestStatus::Valid,
        }
    }
}

impl Default for Request {
    fn default() -> Self {
        Request {
            command:    Cmd::Other(String::new()),
            arguments:  Vec::new(),
            status:     RequestStatus::Pending,
            finished:   false,
            state:      ClientState::default(),
            client_key: Key::default(),
            error:      None,
        }
    }
}
