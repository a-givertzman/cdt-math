use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [bridge control system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CabLocation {
    BridgeEdge,
    BridgeSpanCenter,
    CraneTrolley,
    NoneValue,
}
//
//
impl FromStr for CabLocation {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CabLocation
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bridgeedge" => Ok(Self::BridgeEdge),
            "bridgespancenter" => Ok(Self::BridgeSpanCenter),
            "cranetrolley" => Ok(Self::CraneTrolley),
            "NoneValue" => Ok(Self::NoneValue),
            _ => Err(format!(
                "CabLocation.from_str | Invalid CabLocation: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CabLocation {
    ///
    /// Method translates from enuming structure `CabLocation` into string
    fn to_string(&self) -> String {
        match self {
            CabLocation::BridgeEdge => "BridgeEdge".to_string(),
            CabLocation::BridgeSpanCenter => "BridgeSpanCenter".to_string(),
            CabLocation::CraneTrolley => "CraneTrolley".to_string(),
            CabLocation::NoneValue => "NoneValue".to_string(),
        }
    }
}