use serde::{Serialize, Deserialize};
///
/// TODO!: To be documented
#[derive(Debug, Serialize, Deserialize)]
pub struct TestUserQuery2 {
    pub data: String,
}
//
//
impl TestUserQuery2 {
    pub fn new() -> Self {
        Self {
            data: "TestUserQuery2".to_string(),
        }
    }
}
///
/// Reply to [TestUserQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct TestUserReply2 {
    pub data:String
}
//
//
impl TestUserReply2 {
    pub fn new() -> Self {
        Self {
            data: "TestUserReply2".to_string(),
        }
    }
}
