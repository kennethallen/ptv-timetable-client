use serde::Deserialize;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deserialize)]
pub struct Status {
    pub health: usize,
    pub version: String,
}
