use sal_sync::services::entity::error::str_err::StrErr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
///
/// Represents [hoist perfomance type](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum HoistPerfomanceType {
    #[default]
    Industrial,
    FireSafe,
    ExplosionSafe,
    Marine,
}
//
//
impl FromStr for HoistPerfomanceType {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure HoistPerfomanceType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "industrial" => Ok(Self::Industrial),
            "firesafe" => Ok(Self::FireSafe),
            "explosionsafe" => Ok(Self::ExplosionSafe),
            "marine" => Ok(Self::Marine),
            _ => Err(format!(
                "HoistPerfomanceType.from_str | Invalid HoistPerfomanceType: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for HoistPerfomanceType {
    ///
    /// Method translates from enuming structure `HoistPerfomanceType` into string
    fn to_string(&self) -> String {
        match self {
            HoistPerfomanceType::Industrial => "Industrial".to_string(),
            HoistPerfomanceType::FireSafe => "FireSafe".to_string(),
            HoistPerfomanceType::ExplosionSafe => "ExplosionSafe".to_string(),
            HoistPerfomanceType::Marine => "Marine".to_string(),
        }
    }
}
