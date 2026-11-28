use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [bridge control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BasicCraneControl {
    CraneCab,
    Pendant,
    Remote,
    CabRemote,
}
//
//
impl FromStr for BasicCraneControl {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure BasicCraneControl
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cranecab" => Ok(Self::CraneCab),
            "pendant" => Ok(Self::Pendant),
            "remote" => Ok(Self::Remote),
            "cabremote" => Ok(Self::CabRemote),
            _ => Err(format!(
                "BasicCraneControl.from_str | Invalid BasicCraneControl: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for BasicCraneControl {
    ///
    /// Method translates from enuming structure `BasicCraneControl` into string
    fn to_string(&self) -> String {
        match self {
            BasicCraneControl::CraneCab => "CraneCab".to_string(),
            BasicCraneControl::Pendant => "Pendant".to_string(),
            BasicCraneControl::Remote => "Remote".to_string(),
            BasicCraneControl::CabRemote => "CabRemote".to_string(),
        }
    }
}