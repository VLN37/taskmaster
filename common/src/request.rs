mod class;
mod error;
mod status;

pub use class::Request;
pub use error::{RequestError, Result};
pub use status::RequestStatus;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum ClientState {
    #[default]
    Unattached,
    Attached(String),
}
