use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [crane power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CranePowerSystem {
    Feston,
    CableReel,
    EnergyChain,
    ConductorBar,
}
//
//
impl FromStr for CranePowerSystem {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CranePowerSystem
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "feston" => Ok(Self::Feston),
            "energychain" => Ok(Self::EnergyChain),
            "conductorbar" => Ok(Self::ConductorBar),
            _ => Err(format!(
                "CranePowerSystem.from_str | Invalid CranePowerSystem: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CranePowerSystem {
    ///
    /// Method translates from enuming structure `CranePowerSystem` into string
    fn to_string(&self) -> String {
        match self {
            CranePowerSystem::Feston => "Feston".to_string(),
            CranePowerSystem::CableReel => "CableReel".to_string(),
            CranePowerSystem::EnergyChain => "EnergyChain".to_string(),
            CranePowerSystem::ConductorBar => "ConductorBar".to_string(),
        }
    }
}