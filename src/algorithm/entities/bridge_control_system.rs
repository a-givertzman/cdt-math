use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [bridge control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeControlSystem {
    VFD,
    RCC,
    TCC,
}
//
//
impl FromStr for BridgeControlSystem {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure BridgeControlSystem
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vfd" => Ok(Self::VFD),
            "rcc" => Ok(Self::RCC),
            "tcc" => Ok(Self::TCC),
            _ => Err(format!(
                "BridgeControlSystem.from_str | Invalid BridgeControlSystem: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for BridgeControlSystem {
    ///
    /// Method translates from enuming structure `BridgeControlSystem` into string
    fn to_string(&self) -> String {
        match self {
            BridgeControlSystem::VFD => "VFD".to_string(),
            BridgeControlSystem::RCC => "RCC".to_string(),
            BridgeControlSystem::TCC => "TCC".to_string(),
        }
    }
}