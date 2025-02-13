use serde::{Serialize, Deserialize};
///
/// TODO!: To be documented
#[derive(Debug, Serialize, Deserialize)]
pub struct TestUserQuery1 {
    pub data: String,
}
//
//
impl TestUserQuery1 {
    pub fn new() -> Self {
        Self {
            data: "TestUserQuery1".to_string(),
        }
    }
}
///
/// Reply to [TestUserQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct TestUserReply1 {
    pub data:String
}
//
//
impl TestUserReply1 {
    pub fn new() -> Self {
        Self {
            data: "TestUserReply1".to_string(),
        }
    }
}