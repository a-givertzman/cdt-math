use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [work duration of bridge lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
/// All variables in % - D15 means duration is 15%
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeMovementDuration {
    D15,
    D25,
    D40,
    D60,
    D100,
}
//
//
impl FromStr for BridgeMovementDuration {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure BridgeMovementDuration
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d15" => Ok(Self::D15),
            "d25" => Ok(Self::D25),
            "d40" => Ok(Self::D40),
            "d60" => Ok(Self::D60),
            "d100" => Ok(Self::D100),
            _ => Err(format!(
                "BridgeMovementDuration.from_str | Invalid BridgeMovementDuration: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for BridgeMovementDuration {
    ///
    /// Method translates from enuming structure `BridgeMovementDuration` into string
    fn to_string(&self) -> String {
        match self {
            BridgeMovementDuration::D15 => "D15".to_string(),
            BridgeMovementDuration::D25 => "D25".to_string(),
            BridgeMovementDuration::D40 => "D40".to_string(),
            BridgeMovementDuration::D60 => "D60".to_string(),
            BridgeMovementDuration::D100 => "D100".to_string(),
        }
    }
}