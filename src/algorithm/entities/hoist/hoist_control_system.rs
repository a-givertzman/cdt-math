use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [hoist control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoistControlSystem {
    VFD,
    RCC,
    TCC,
}
//
//
impl FromStr for HoistControlSystem {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistControlSystem
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vfd" => Ok(Self::VFD),
            "rcc" => Ok(Self::RCC),
            "tcc" => Ok(Self::TCC),
            _ => Err(format!(
                "HoistGroup.from_str | Invalid HoistGroup: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistControlSystem {
    ///
    /// Method translates from enuming structure `HoistControlSystem` into string
    fn to_string(&self) -> String {
        match self {
            HoistControlSystem::VFD => "VFD".to_string(),
            HoistControlSystem::RCC => "RCC".to_string(),
            HoistControlSystem::TCC => "TCC".to_string(),
        }
    }
}