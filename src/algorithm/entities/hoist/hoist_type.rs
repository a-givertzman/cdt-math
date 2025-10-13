use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [hoist type](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter02_typesHoists.md)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum HoistType {
    #[default]
    Rope,
    Chain,
}
//
//
impl FromStr for HoistType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rope" => Ok(Self::Rope),
            "chain" => Ok(Self::Chain),
            _ => Err(format!(
                "HoistType.from_str | Invalid HoistType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistType {
    ///
    /// Method translates from enuming structure `HoistType` into string
    fn to_string(&self) -> String {
        match self {
            HoistType::Rope => "Rope".to_string(),
            HoistType::Chain => "Chain".to_string(),
        }
    }
}