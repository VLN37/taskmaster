mod class;
mod error;

pub use class::Server;
pub use error::{Result, ServerError};

pub type Key = u64;
pub const SERVER_KEY: Key = 42;
