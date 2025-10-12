use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [hoist type](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter02_typesHoists.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoistMobilityType {
    Stationary,
    MobileSuspended,
    MobileSupport,
    NormalHeadroom,
    ReducedHeadroom
}
//
//
impl FromStr for HoistMobilityType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistMobilityType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "Stationary" => Ok(Self::Stationary),
            "MobileSuspended" => Ok(Self::MobileSuspended),
            "MobileSupport" => Ok(Self::MobileSupport),
            "NormalHeadroom" => Ok(Self::NormalHeadroom),
            "ReducedHeadroom" => Ok(Self::ReducedHeadroom),
            _ => Err(format!(
                "HoistMobilityType.from_str | Invalid HoistMobilityType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistMobilityType {
    ///
    /// Method translates from enuming structure `HoistMobilityType` into string
    fn to_string(&self) -> String {
        match self {
            HoistMobilityType::Stationary => "Stationary".to_string(),
            HoistMobilityType::MobileSuspended => "MobileSuspended".to_string(),
            HoistMobilityType::MobileSupport => "MobileSupport".to_string(),
            HoistMobilityType::NormalHeadroom => "NormalHeadroom".to_string(),
            HoistMobilityType::ReducedHeadroom => "ReducedHeadroom".to_string(),
        }
    }
}