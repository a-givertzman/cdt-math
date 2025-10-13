use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [hoist headroom type](https://github.com/a-givertzman/cdt-math/blob/Docs-hoist-mechanism-Hoist/design/docs/algorithm_single_ginger_overhead_crane/part02_hoistMechanism/chapter02_typesHoists.md)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum HoistHeadroomType {
    #[default]
    Normal,
    Deacreased,
    Any,
}
//
//
impl FromStr for HoistHeadroomType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistHeadroomType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Self::Normal),
            "deacreased" => Ok(Self::Deacreased),
            "any" => Ok(Self::Any),
            _ => Err(format!(
                "HoistHeadroomType.from_str | Invalid HoistHeadroomType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistHeadroomType {
    ///
    /// Method translates from enuming structure `HoistHeadroomType` into string
    fn to_string(&self) -> String {
        match self {
            HoistHeadroomType::Normal => "Normal".to_string(),
            HoistHeadroomType::Deacreased => "Deacreased".to_string(),
            HoistHeadroomType::Any => "Any".to_string(),
        }
    }
}