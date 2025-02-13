use serde::{Deserialize, Serialize};

///
/// List of all possible requests in Client-Server interface
#[derive(Debug, Serialize, Deserialize)]
pub enum QueryKind {
    ///
    /// Used for example only, to be deleted
    TestUserQuery,
}