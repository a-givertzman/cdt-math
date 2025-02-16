use serde::{Deserialize, Serialize};
use super::{rope_durability_class::RopeDurabilityClass, rope_type::RopeType};
///
/// Struct to describe a hoisting rope
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HoistingRope {
    /// Full name of rope
    pub name: String,
    /// rope diameter
    pub rope_diameter: f64,
    /// type of rope
    pub r#type: RopeType,
    /// class of rope durability
    pub rope_durability: RopeDurabilityClass,
    /// rope breaking force
    pub rope_force: f64,
    /// rope cross-sectional area
    pub s: f64,
    /// specific gravity of rope
    pub m: f64
}
