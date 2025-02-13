use serde::{Serialize, Deserialize};
///
/// TODO!: To be documented
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserHookQuery {
    pub data: String,
}
//
//
impl ChooseUserHookQuery {
    pub fn new() -> Self {
        Self {
            data: "ChooseUserHookQuery".to_string(),
        }
    }
}
///
/// Reply to [ChooseUserHookQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChooseUserHookReply {
    pub data:String
}
//
//
impl ChooseUserHookReply {
    pub fn new() -> Self {
        Self {
            data: "ChooseUserHookReply".to_string(),
        }
    }
}