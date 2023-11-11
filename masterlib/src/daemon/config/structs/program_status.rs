use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ProgramStatus {
    Starting,
    FailedToStart,
    Active,
    GracefulExit,
    Killed,
    FailedExit,
}
