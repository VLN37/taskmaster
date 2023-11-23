use std::fmt;

#[derive(Debug, Clone)]
pub struct ImproperlyConfigured;

impl fmt::Display for ImproperlyConfigured {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "invalid option") }
}
