use core::fmt;

#[derive(Debug)]
pub struct RequestError {
    message: String,
}

impl std::error::Error for RequestError {}
impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub type Result<T> = std::result::Result<T, RequestError>;
