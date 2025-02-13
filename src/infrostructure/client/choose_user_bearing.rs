use serde::{Serialize, Deserialize};
///
/// TODO!: To be documented
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserBearingQuery {
    pub data: String,
}
//
//
impl ChooseUserBearingQuery {
    pub fn new() -> Self {
        Self {
            data: "ChooseUserBearingQuery".to_string(),
        }
    }
}
///
/// Reply to [ChooseUserBearingQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserBearingReply {
    pub data: String
}
//
//
impl ChooseUserBearingReply {
    pub fn new() -> Self {
        Self {
            data: "ChooseUserBearingReply".to_string(),
        }
    }
}
