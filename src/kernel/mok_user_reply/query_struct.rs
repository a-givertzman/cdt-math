use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryStruct {
    pub data: String,
}

impl QueryStruct {
    pub fn new() -> Self {
        Self {
            data: "Test query".to_string(),
        }
    }
}
