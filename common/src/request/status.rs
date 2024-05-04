#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum RequestStatus {
    #[default]
    Pending,
    Valid,
    Invalid,
}

impl From<RequestStatus> for bool {
    fn from(value: RequestStatus) -> bool { value == RequestStatus::Valid }
}
