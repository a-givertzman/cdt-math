use serde::{Deserialize, Serialize};
///
/// Struct to describe a bearing
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Bearing {
    pub name: String,
    pub outer_diameter: f64,
    pub inner_diameter: f64,
    pub static_load_capacity: f64,
    pub height: f64,
}