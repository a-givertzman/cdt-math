use std::str::FromStr;

use api_tools::error::str_err::StrErr;
use serde::{
    Deserialize, 
    Serialize
};
///
/// [work duration of lifting mechanism](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
/// All variables in % - D15 means duration is 15%
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LiftingDuration {
    D15,
    D25,
    D40,
    D60,
    D100,
}
//
//
impl FromStr for LiftingDuration {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure LiftingDuration
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "d15" => Ok(Self::D15),
            "d25" => Ok(Self::D25),
            "d40" => Ok(Self::D40),
            "d60" => Ok(Self::D60),
            "d100" => Ok(Self::D100),
            _ => Err(format!(
                "LiftingDuration.from_str | Invalid LiftingDuration: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for LiftingDuration {
    ///
    /// Method translates from enuming structure `LiftingDuration` into string
    fn to_string(&self) -> String {
        match self {
            LiftingDuration::D15 => "D15".to_string(),
            LiftingDuration::D25 => "D25".to_string(),
            LiftingDuration::D40 => "D40".to_string(),
            LiftingDuration::D60 => "D60".to_string(),
            LiftingDuration::D100 => "D100".to_string(),
        }
    }
}