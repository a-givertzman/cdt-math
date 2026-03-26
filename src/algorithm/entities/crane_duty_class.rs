use sal_sync::services::entity::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
use std::str::FromStr;
///
/// Represents [crane duty class](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CraneDutyClass {
    A0, 
    A1, 
    A2, 
    A3, 
    A4, 
    A5, 
    A6, 
    A7, 
    A8, 
    A9, 
    A10,
    A11,
}
//
//
impl FromStr for CraneDutyClass {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure CraneDutyClass
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a0" => Ok(Self::A0),
            "a1" => Ok(Self::A1),
            "a2" => Ok(Self::A2),
            "a3" => Ok(Self::A3),
            "a4" => Ok(Self::A4),
            "a5" => Ok(Self::A5),
            "a6" => Ok(Self::A6),
            "a7" => Ok(Self::A7),
            "a8" => Ok(Self::A8),
            "a9" => Ok(Self::A9),
            "a10" => Ok(Self::A10),
            "a11" => Ok(Self::A11),
            _ => Err(format!(
                "CraneDutyClass.from_str | Invalid CraneDutyClass: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for CraneDutyClass {
    ///
    /// Method translates from enuming structure `CraneDutyClass` into string
    fn to_string(&self) -> String {
        match self {
            CraneDutyClass::A0 => "A0".to_string(),
            CraneDutyClass::A1 => "A1".to_string(),
            CraneDutyClass::A2 => "A2".to_string(),
            CraneDutyClass::A3 => "A3".to_string(),
            CraneDutyClass::A4 => "A4".to_string(),
            CraneDutyClass::A5 => "A5".to_string(),
            CraneDutyClass::A6 => "A6".to_string(),
            CraneDutyClass::A7 => "A7".to_string(),
            CraneDutyClass::A8 => "A8".to_string(),
            CraneDutyClass::A9 => "A9".to_string(),
            CraneDutyClass::A10 => "A10".to_string(),
            CraneDutyClass::A11 => "A11".to_string(),

        }
    }
}
