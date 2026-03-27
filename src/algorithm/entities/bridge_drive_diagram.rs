use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [bridge drive diagram](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeDriveDiagram {
    Central,
    Separate,
}
//
//
impl FromStr for BridgeDriveDiagram {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure BridgeDriveDiagram
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "central" => Ok(Self::Central),
            "separate" => Ok(Self::Separate),
            _ => Err(format!(
                "BridgeDriveDiagram.from_str | Invalid BridgeDriveDiagram: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for BridgeDriveDiagram {
    ///
    /// Method translates from enuming structure `BridgeDriveDiagram` into string
    fn to_string(&self) -> String {
        match self {
            BridgeDriveDiagram::Central => "Central".to_string(),
            BridgeDriveDiagram::Separate => "Separate".to_string(),
        }
    }
}