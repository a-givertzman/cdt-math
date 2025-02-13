use serde::{Serialize, Deserialize};
///
/// Struct to describe type of user request, that ascs user for choosing hoisting rope from filtered
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseHoistingRopeQuery {
    pub data: String,
}
//
//
impl ChooseHoistingRopeQuery {
    pub fn new() -> Self {
        Self {
            data: "ChooseHoistingRopeQuery".to_string(),
        }
    }
}
///
/// Reply to [ChooseHoistingRopeQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseHoistingRopeReply {
    pub data: String
}
//
//
impl ChooseHoistingRopeReply {
    pub fn new() -> Self {
        Self {
            data: "ChooseHoistingRopeReply".to_string(),
        }
    }
}
