use sal_sync::services::entity::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
use std::str::FromStr;
///
/// Represents [crane climatic category](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CraneClimaticCategory {
    Y1,
    Y2,
    Y3,
    Y5,
    HL1,
    HL2,
    HL3,
    YHL4,
    YHL41,
    YHL42,
    O4,
    O41,
    O42,
    T5,
    TC3,
    B3,
    B31,
    B41,
    OM4,
    B5,
}
//
//
impl FromStr for CraneClimaticCategory {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CraneClimaticCategory
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "Y1" => Ok(Self::Y1),
            "Y2" => Ok(Self::Y2),
            "Y3" => Ok(Self::Y3),
            "Y5" => Ok(Self::Y5),
            "HL1"=> Ok(Self::HL1),
            "HL2" => Ok(Self::HL2),
            "HL3" => Ok(Self::HL3),
            "YHL4" => Ok(Self::YHL4),
            "YHL41" => Ok(Self::YHL41),
            "YHL42" => Ok(Self::YHL42),
            "O4" => Ok(Self::O4),
            "O41" => Ok(Self::O41),
            "O42" => Ok(Self::O42),
            "T5" => Ok(Self::T5),
            "TC3" => Ok(Self::TC3),
            "B3" => Ok(Self::B3),
            "B31" => Ok(Self::B31),
            "B41" => Ok(Self::B41),
            "OM4" => Ok(Self::OM4),
            "B5" => Ok(Self::B5),
            _ => Err(format!(
                "CraneClimaticCategory.from_str | Invalid CraneClimaticCategory: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CraneClimaticCategory {
    ///
    /// Method translates from enuming structure `CraneClimaticCategory` into string
    fn to_string(&self) -> String {
        match self {
            CraneClimaticCategory::Y1 => "Y1".to_string(),
            CraneClimaticCategory::Y2 => "Y2".to_string(),
            CraneClimaticCategory::Y3 => "Y3".to_string(),
            CraneClimaticCategory::Y5 => "Y5".to_string(),
            CraneClimaticCategory::HL1 => "HL1".to_string(),
            CraneClimaticCategory::HL2 => "HL2".to_string(),
            CraneClimaticCategory::HL3 => "HL3".to_string(),
            CraneClimaticCategory::YHL4 => "YHL4".to_string(),
            CraneClimaticCategory::YHL41 => "YHL41".to_string(),
            CraneClimaticCategory::YHL42 => "YHL42".to_string(),
            CraneClimaticCategory::O4 => "O4".to_string(),
            CraneClimaticCategory::O41 => "O41".to_string(),
            CraneClimaticCategory::O42 => "O42".to_string(),
            CraneClimaticCategory::T5 => "T5".to_string(),
            CraneClimaticCategory::TC3 => "TC3".to_string(),
            CraneClimaticCategory::B3 => "B3".to_string(),
            CraneClimaticCategory::B31 => "B31".to_string(),
            CraneClimaticCategory::B41 => "B41".to_string(),
            CraneClimaticCategory::OM4 => "OM4".to_string(),
            CraneClimaticCategory::B5 => "B5".to_string(),

        }
    }
}
