use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [crane power system](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum UsageClass {
        #[default]
        T0,
        T1,
        T2,
        T3,
        T4,
        T5,
        T6,
        T7,
        T8,
        T9,
}
//
//
impl FromStr for UsageClass {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure UsageClass
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "t0" => Ok(Self::T0),
            "t1" => Ok(Self::T1),
            "t2" => Ok(Self::T2),
            "t3" => Ok(Self::T3),
            "t4" => Ok(Self::T4),
            "t5" => Ok(Self::T5),
            "t6" => Ok(Self::T6),
            "t7" => Ok(Self::T7),
            "t8" => Ok(Self::T8),
            "t9" => Ok(Self::T9),
            _ => Err(format!(
                "UsageClass.from_str | Invalid UsageClass: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for UsageClass {
    ///
    /// Method translates from enuming structure `UsageClass` into string
    fn to_string(&self) -> String {
        match self {
            UsageClass::T0 => "T0".to_string(),
            UsageClass::T1 => "T1".to_string(),
            UsageClass::T2 => "T2".to_string(),
            UsageClass::T3 => "T3".to_string(),
            UsageClass::T4 => "T4".to_string(),
            UsageClass::T5 => "T5".to_string(),
            UsageClass::T6 => "T6".to_string(),
            UsageClass::T7 => "T7".to_string(),
            UsageClass::T8 => "T8".to_string(),
            UsageClass::T9 => "T9".to_string(),
        }
    }
}