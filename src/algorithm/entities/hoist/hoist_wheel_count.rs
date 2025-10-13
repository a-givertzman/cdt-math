use sal_sync::services::entity::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
use std::str::FromStr;
///
/// Represents [hoist wheel count](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum HoistWheelCount {
    #[default]
    Four,
    Eight,
    Any,
}
//
//
impl FromStr for HoistWheelCount {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistWheelCount
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "four" => Ok(Self::Four),
            "eight" => Ok(Self::Eight),
            "any" => Ok(Self::Any),
            _ => Err(format!(
                "HoistWheelCount.from_str | Invalid HoistWheelCount: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistWheelCount {
    ///
    /// Method translates from enuming structure `HoistWheelCount` into string
    fn to_string(&self) -> String {
        match self {
            HoistWheelCount::Four => "Four".to_string(),
            HoistWheelCount::Eight => "Eight".to_string(),
            HoistWheelCount::Any => "Any".to_string(),
        }
    }
}
