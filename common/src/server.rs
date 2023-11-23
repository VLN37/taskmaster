mod request;
mod request_factory;
mod server_class;

pub use request::Request;
pub use request_factory::RequestFactory;
pub use server_class::Server;

pub type Key = u64;
pub const SERVER_KEY: Key = 42;
