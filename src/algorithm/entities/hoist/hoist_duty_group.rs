use sal_sync::services::entity::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
use std::str::FromStr;
///
/// Represents [hoist duty group](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum HoistDutyGroup {
    #[default]
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
    M9,
}
//
//
impl FromStr for HoistDutyGroup {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistDutyGroup
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "m1" => Ok(Self::M1),
            "m2" => Ok(Self::M2),
            "m3" => Ok(Self::M3),
            "m4" => Ok(Self::M4),
            "m5" => Ok(Self::M5),
            "m6" => Ok(Self::M6),
            "m7" => Ok(Self::M7),
            "m8" => Ok(Self::M8),
            "m9" => Ok(Self::M9),
            _ => Err(format!(
                "HoistDutyGroup.from_str | Invalid HoistDutyGroup: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistDutyGroup {
    ///
    /// Method translates from enuming structure `HoistDutyGroup` into string
    fn to_string(&self) -> String {
        match self {
            HoistDutyGroup::M1 => "M1".to_string(),
            HoistDutyGroup::M2 => "M2".to_string(),
            HoistDutyGroup::M3 => "M3".to_string(),
            HoistDutyGroup::M4 => "M4".to_string(),
            HoistDutyGroup::M5 => "M5".to_string(),
            HoistDutyGroup::M6 => "M6".to_string(),
            HoistDutyGroup::M7 => "M7".to_string(),
            HoistDutyGroup::M8 => "M8".to_string(),
            HoistDutyGroup::M9 => "M9".to_string(),
        }
    }
}
