use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [trolley power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrolleyPowerSystem {
    Feston,
    EnergyChain,
    ConductorBar,
}
//
//
impl FromStr for TrolleyPowerSystem {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure TrolleyPowerSystem
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "feston" => Ok(Self::Feston),
            "energychain" => Ok(Self::EnergyChain),
            "conductorbar" => Ok(Self::ConductorBar),
            _ => Err(format!(
                "TrolleyPowerSystem.from_str | Invalid TrolleyPowerSystem: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for TrolleyPowerSystem {
    ///
    /// Method translates from enuming structure `TrolleyPowerSystem` into string
    fn to_string(&self) -> String {
        match self {
            TrolleyPowerSystem::Feston => "Feston".to_string(),
            TrolleyPowerSystem::EnergyChain => "EnergyChain".to_string(),
            TrolleyPowerSystem::ConductorBar => "ConductorBar".to_string(),
        }
    }
}