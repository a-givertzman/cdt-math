use std::str::FromStr;
use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [crane type](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraneType {
    Industrial,
    Metallurgical,
    Special,
}
//
//
impl FromStr for CraneType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CraneType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "industrial" => Ok(Self::Industrial),
            "metallurgical" => Ok(Self::Metallurgical),
            "special" => Ok(Self::Special),
            _ => Err(format!(
                "CraneType.from_str | Invalid CraneType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CraneType {
    ///
    /// Method translates from enuming structure `CraneType` into string
    fn to_string(&self) -> String {
        match self {
            CraneType::Industrial => "Industrial".to_string(),
            CraneType::Metallurgical => "Metallurgical".to_string(),
            CraneType::Special => "Special".to_string(),
        }
    }
}