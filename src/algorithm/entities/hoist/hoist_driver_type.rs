use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [hoist type](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter02_typesHoists.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HoistDriverType {
    Electric,
    Handed,
}
//
//
impl FromStr for HoistDriverType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistDriverType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "electric" => Ok(Self::Electric),
            "handed" => Ok(Self::Handed),
            _ => Err(format!(
                "HoistDriverType.from_str | Invalid HoistDriverType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistDriverType {
    ///
    /// Method translates from enuming structure `HoistDriverType` into string
    fn to_string(&self) -> String {
        match self {
            HoistDriverType::Electric => "Electric".to_string(),
            HoistDriverType::Handed => "Handed".to_string(),
        }
    }
}