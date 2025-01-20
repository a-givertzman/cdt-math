use serde::{Deserialize, Serialize};
///
/// Struct to describe a hook
/// - 'gost' - GOST number of hook
/// - 'type' - hook type
/// - 'load_capacity_m13' - loading capacity for M1-M3 types of mechanism work
/// - 'load_capacity_m46' - loading capacity for M4-M6 types of mechanism work
/// - 'load_capacity_m78' - loading capacity for M7-M8 types of mechanism work
/// - 'shank_diameter' - shank diameter for bearing
/// - 'weight' - weight of the hook
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Hook {
    pub gost: String,
    pub r#type: String,
    pub load_capacity_m13: f64,
    pub load_capacity_m46: f64,
    pub load_capacity_m78: f64,
    pub shank_diameter: f64,
    pub weight: f64,
}