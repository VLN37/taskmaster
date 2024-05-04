use core::fmt;

use super::CmdError;

#[derive(Debug, Clone)]
pub enum CmdErrorKind {
    NotFound(String),
    InvalidArguments,
}

impl fmt::Display for CmdErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self.clone()))
    }
}

impl From<CmdErrorKind> for String {
    fn from(value: CmdErrorKind) -> Self {
        match value {
            CmdErrorKind::InvalidArguments => {
                "ATTACH requires a program argument".into()
            }
            CmdErrorKind::NotFound(program) => {
                format!("{program} is not a Taskmaster Program")
            }
        }
    }
}

impl From<CmdErrorKind> for CmdError {
    fn from(value: CmdErrorKind) -> Self {
        CmdError {
            message: value.into(),
        }
    }
}
