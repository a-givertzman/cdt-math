use serde::{Deserialize, Serialize};
///
/// Struct to describe an another loading handing device
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnLoadDevice {
    pub name: String,
    pub weight: f64,
}