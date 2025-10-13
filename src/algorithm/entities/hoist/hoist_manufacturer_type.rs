use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [hoist manufacturer type](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter02_typesHoists.md)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum HoistManufacturerType {
    #[default]
    Nante,
    BalkanskoEcho,
    Stahl,
}
//
//
impl FromStr for HoistManufacturerType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistManufacturerType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nante" => Ok(Self::Nante),
            "balkanskoecho" => Ok(Self::BalkanskoEcho),
            "stahl" => Ok(Self::Stahl),
            _ => Err(format!(
                "HoistManufacturerType.from_str | Invalid HoistManufacturerType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistManufacturerType {
    ///
    /// Method translates from enuming structure `HoistManufacturerType` into string
    fn to_string(&self) -> String {
        match self {
            HoistManufacturerType::Nante => "Nante".to_string(),
            HoistManufacturerType::BalkanskoEcho => "BalkanskoEcho".to_string(),
            HoistManufacturerType::Stahl => "Stahl".to_string(),
        }
    }
}