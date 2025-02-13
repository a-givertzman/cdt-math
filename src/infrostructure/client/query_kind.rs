use std::str::FromStr;
use sal_sync::services::entity::error::str_err::StrErr;
use serde::{Deserialize, Serialize};

///
/// List of all possible requests in Client-Server interface
#[derive(Debug, Serialize, Deserialize)]
pub enum QueryKind {
    ///
    /// Used for example only, to be deleted
    TestUserQuery1,
    ///
    /// Used for example only, to be deleted
    TestUserQuery2,
}
//
//
impl FromStr for QueryKind {
    type Err = StrErr;
    //
    //
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "test-user-query-1" => {
                Ok(QueryKind::TestUserQuery1)
            }
            "test-user-query-2" => {
                Ok(QueryKind::TestUserQuery2)
            }
            _ => Err(StrErr(
                format!("{}.run | Unknown kind of query: {}", std::any::type_name::<Self>(), s),
            )),
        }
    }
}