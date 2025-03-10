use crate::kernel::str_err::str_err::StrErr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
///
/// Represents [crane work area types](design\docs\algorithm\part01\initial_data.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraneWorkArea {
    Default,
    Aggressive,
    StrongAggressive,
}
//
//
impl FromStr for CraneWorkArea {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CraneWorkArea
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Self::Default),
            "aggressive" => Ok(Self::Aggressive),
            "strongaggressive" => Ok(Self::StrongAggressive),
            _ => Err(format!("CraneWorkArea.from_str | Invalid CraneWorkArea: {}", s).into()),
        }
    }
}
