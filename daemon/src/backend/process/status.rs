use crate::config::Signal;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ProcessStatus {
    Starting,
    FailedToStart,
    Active,
    GracefulExit(u32),
    Killed(Signal),
    FailedExit(u32),
}

impl std::fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{:?}", self))
    }
}
