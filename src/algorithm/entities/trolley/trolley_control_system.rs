use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [trolley control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrolleyControlSystem {
    VFD,
    RCC,
    TCC,
}
//
//
impl FromStr for TrolleyControlSystem {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure TrolleyControlSystem
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "vfd" => Ok(Self::VFD),
            "rcc" => Ok(Self::RCC),
            "tcc" => Ok(Self::TCC),
            _ => Err(format!(
                "TrolleyControlSystem.from_str | Invalid TrolleyControlSystem: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for TrolleyControlSystem {
    ///
    /// Method translates from enuming structure `TrolleyControlSystem` into string
    fn to_string(&self) -> String {
        match self {
            TrolleyControlSystem::VFD => "VFD".to_string(),
            TrolleyControlSystem::RCC => "RCC".to_string(),
            TrolleyControlSystem::TCC => "TCC".to_string(),
        }
    }
}