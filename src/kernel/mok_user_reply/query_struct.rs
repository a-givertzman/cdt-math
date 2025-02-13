use serde::{Serialize, Deserialize};
use crate::infrostructure::client::query_kind::QueryKind;
///
/// TODO!: To be documented
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryStruct {
    pub kind: QueryKind,
    pub data: String,
}
//
//
impl QueryStruct {
    pub fn new(kind: QueryKind) -> Self {
        Self {
            kind,
            data: "Test query".to_string(),
        }
    }
}
