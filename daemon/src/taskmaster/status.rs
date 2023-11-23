#[derive(Default, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Status {
    #[default]
    Starting,
    Reloading,
    Active,
}
