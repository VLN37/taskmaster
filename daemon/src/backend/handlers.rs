use common::request::{ClientState, Request};
use common::{Cmd, CmdError, CmdErrorKind, CmdHandler};

use crate::BackEnd;

impl CmdHandler for BackEnd {
    fn handle(&mut self, request: &mut Request) -> Result<String, CmdError> {
        match &request.command {
            Cmd::Log => self.log(request),
            Cmd::Status => self.status(request),
            Cmd::Head => self.head(request),
            Cmd::Attach => self.attach(request),
            Cmd::Other(_) => self.other(request),
        }
    }

    fn attach(&mut self, request: &mut Request) -> Result<String, CmdError> {
        request.finished = true;

        let command_name = match request.arguments.first() {
            Some(value) => value,
            None => {
                let kind = CmdErrorKind::InvalidArguments;
                return Err(format!("Attach failed: {kind}").into());
            }
        };
        if !self.programs.contains_key(command_name) {
            let kind = CmdErrorKind::NotFound(command_name.into());
            return Err(format!("Attach failed: {kind}").into());
        }

        let state = ClientState::Attached(command_name.into());
        request.state = state.clone();
        let client = self.clients.get_mut(&request.client_key).unwrap();
        client.state = state.clone();
        Ok("Attach successful!".into())
    }

    fn log(&self, request: &mut Request) -> Result<String, CmdError> {
        request.finished = true;
        Ok("todo!(log)".into())
    }

    fn head(&self, request: &mut Request) -> Result<String, CmdError> {
        request.finished = true;
        Ok("todo!(head)".into())
    }

    fn status(&self, request: &mut Request) -> Result<String, CmdError> {
        request.finished = true;
        Ok("todo!(status)".into())
    }

    fn other(&self, request: &mut Request) -> Result<String, CmdError> {
        request.finished = true;
        Ok("todo!(other)".into())
    }
}