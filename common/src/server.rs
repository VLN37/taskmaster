mod server_class;
mod server_error;

pub use server_class::Server;
pub use server_error::{Result, ServerError};

pub type Key = u64;
pub const SERVER_KEY: Key = 42;
