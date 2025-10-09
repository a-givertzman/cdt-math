use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [crane purpose](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CranePurpose {
    Industrial,
    Metallurgical,
    Special,
    Marine,
}
//
//
impl FromStr for CranePurpose {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CranePurpose
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "industrial" => Ok(Self::Industrial),
            "metallurgical" => Ok(Self::Metallurgical),
            "special" => Ok(Self::Special),
            "marine" => Ok(Self::Marine),
            _ => Err(format!(
                "CranePurpose.from_str | Invalid CranePurpose: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CranePurpose {
    ///
    /// Method translates from enuming structure `CranePurpose` into string
    fn to_string(&self) -> String {
        match self {
            CranePurpose::Industrial => "Industrial".to_string(),
            CranePurpose::Metallurgical => "Metallurgical".to_string(),
            CranePurpose::Special => "Special".to_string(),
            CranePurpose::Marine => "Marine".to_string(),
        }
    }
}