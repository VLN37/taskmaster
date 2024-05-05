use crate::server::{Key, ServerError};
use crate::{ClientState, Cmd, RequestStatus};

#[derive(Debug, Clone)]
pub struct Request {
    pub command:    Cmd,
    pub arguments:  Vec<String>,
    pub status:     RequestStatus,
    pub finished:   bool,
    pub state:      ClientState,
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
            RequestStatus::Pending => self.validate().unwrap_or(false),
        }
    }

    pub fn validate(&mut self) -> Result<bool, ServerError> {
        self.validate_command()?;
        Ok(true)
    }

    fn validate_command(&mut self) -> Result<bool, ServerError> {
        match &self.command {
            Cmd::Other(_) => match &self.state {
                ClientState::Attached(_) => {
                    self.status = RequestStatus::Valid;
                    Ok(true)
                }
                ClientState::Unattached => {
                    self.status = RequestStatus::Invalid;
                    let err = format!("Invalid command: {}", self.command);
                    Err(ServerError::new(&err))
                }
            },
            _ => {
                self.status = RequestStatus::Valid;
                Ok(true)
            }
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
        }
    }
}
