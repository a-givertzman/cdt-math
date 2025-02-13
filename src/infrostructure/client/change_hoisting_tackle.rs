use serde::{Serialize, Deserialize};
///
/// Struct to describe type of user request, that ascs user for changing hoisting tackle if it needed
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeHoistingTackleQuery {
    pub data: String,
}
//
//
impl ChangeHoistingTackleQuery {
    pub fn new() -> Self {
        Self {
            data: "ChangeHoistingTackleQuery".to_string(),
        }
    }
}
///
/// Reply to [ChangeHoistingTackleQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeHoistingTackleReply {
    pub data: String
}
//
//
impl ChangeHoistingTackleReply {
    pub fn new() -> Self {
        Self {
            data: "ChangeHoistingTackleReply".to_string(),
        }
    }
}
