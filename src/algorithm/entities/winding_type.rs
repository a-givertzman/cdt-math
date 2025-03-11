use sal_sync::services::entity::error::str_err::StrErr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
///
/// Represents [winding types](design\docs\algorithm\part01\initial_data.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WindingType {
    SingleLayer,
    MultiLayer,
}
//
//
impl FromStr for WindingType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure WindingType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "singlelayer" => Ok(Self::SingleLayer),
            "multilayer" => Ok(Self::MultiLayer),
            _ => Err(format!("WindingType.from_str | Invalid WindingType: {}", s).into()),
        }
    }
}
