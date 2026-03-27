use sal_sync::services::entity::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
use std::str::FromStr;
///
/// Represents [crane wind area](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CraneWindArea {
    O,
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    Sea,
}
//
//
impl FromStr for CraneWindArea {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CraneWindArea
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "o" => Ok(Self::O),
            "i" => Ok(Self::I),
            "ii" => Ok(Self::II),
            "iii" => Ok(Self::III),
            "iv" => Ok(Self::IV),
            "v" => Ok(Self::V),
            "vi" => Ok(Self::VI),
            "vii" => Ok(Self::VII),
            "sea" => Ok(Self::Sea),
            _ => Err(format!(
                "CraneWindArea.from_str | Invalid CraneWindArea: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CraneWindArea {
    ///
    /// Method translates from enuming structure `CraneWindArea` into string
    fn to_string(&self) -> String {
        match self {
            CraneWindArea::O => "O".to_string(),
            CraneWindArea::I => "I".to_string(),
            CraneWindArea::II => "II".to_string(),
            CraneWindArea::III => "III".to_string(),
            CraneWindArea::IV => "IV".to_string(),
            CraneWindArea::V => "V".to_string(),
            CraneWindArea::VI => "VI".to_string(),
            CraneWindArea::VII => "VII".to_string(),
            CraneWindArea::Sea => "Sea".to_string(),
        }
    }
}
