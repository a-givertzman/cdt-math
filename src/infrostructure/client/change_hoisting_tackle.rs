use serde::{Serialize, Deserialize};
///
/// Struct to describe type of user request, that ascs user for changing hoisting tackle if it needed
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeHoistingTackleQuery {
    pub hoisting_tackle: Vec<u8>,
}
//
//
impl ChangeHoistingTackleQuery {
    pub fn new() -> Self {
        Self {
            hoisting_tackle: vec![1,2],
        }
    }
}
///
/// Reply to [ChangeHoistingTackleQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChangeHoistingTackleReply {
    pub choosen_hoisting_tackle: u8
}
//
//
impl ChangeHoistingTackleReply {
    pub fn new(choosen_hoisting_tackle: u8) -> Self {
        Self {
            choosen_hoisting_tackle,
        }
    }
}
