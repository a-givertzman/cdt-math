use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [bridge drive system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeDriveSystem {
    DetailedBridgeDrive,
    GearMotor,
}
//
//
impl FromStr for BridgeDriveSystem {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure BridgeDriveSystem
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "detailedbridgedrive" => Ok(Self::DetailedBridgeDrive),
            "gearmotor" => Ok(Self::GearMotor),
            _ => Err(format!(
                "BridgeDriveSystem.from_str | Invalid BridgeDriveSystem: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for BridgeDriveSystem {
    ///
    /// Method translates from enuming structure `BridgeDriveSystem` into string
    fn to_string(&self) -> String {
        match self {
            BridgeDriveSystem::DetailedBridgeDrive => "DetailedBridgeDrive".to_string(),
            BridgeDriveSystem::GearMotor => "GearMotor".to_string(),
        }
    }
}