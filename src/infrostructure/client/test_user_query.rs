use serde::{Serialize, Deserialize};
///
/// TODO!: To be documented
#[derive(Debug, Serialize, Deserialize)]
pub struct TestUserQuery {
    pub data: String,
}
//
//
impl TestUserQuery {
    pub fn new() -> Self {
        Self {
            data: "Test query".to_string(),
        }
    }
}
///
/// Reply to [TestUserQuery]
#[derive(Debug, Serialize, Deserialize)]
pub struct TestUserReply {
    pub data:String
}