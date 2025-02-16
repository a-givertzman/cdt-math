use serde::{Serialize, Deserialize};

use crate::algorithm::entities::hoisting_rope::{hoisting_rope::HoistingRope, rope_durability_class::RopeDurabilityClass, rope_type::RopeType};
///
/// Struct to describe type of user request, that ascs user for choosing hoisting rope from filtered
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseHoistingRopeQuery {
    /// hoisting ropes filtered by user characteristics
    pub filtered_hoisting_ropes: Vec<HoistingRope>,
}
//
//
impl ChooseHoistingRopeQuery {
    pub fn new(filtered_hoisting_ropes: Vec<HoistingRope>) -> Self {
        Self {
            filtered_hoisting_ropes,
        }
    }
}
///
/// Reply to [ChooseHoistingRopeQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseHoistingRopeReply {
    pub choosen_rope: HoistingRope
}
//
//
impl ChooseHoistingRopeReply {
    pub fn new(choosen_rope: HoistingRope) -> Self {
        Self {
            choosen_rope,
        }
    }
}
