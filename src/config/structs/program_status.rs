use serde::{Deserialize, Serialize};
pub use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ProgramStatus {
    Starting,
    FailedToStart,
    Active,
    GracefulExit,
    Killed,
    FailedExit,
}
