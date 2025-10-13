
use sal_sync::services::entity::error::str_err::StrErr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
///
/// Represents [explosion-fire-safe crane purpose](design/docs/algorithm_single_ginger_overhead_crane/part01_initialization/chapter01_initialData/chapter01_initialData.md)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ExplosionFireSaveCranePurpose {
    Industrial,
    FireSafe,
    ExplosionSafe,
    Marine,
}
//
//
impl FromStr for ExplosionFireSaveCranePurpose {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure ExplosionFireSaveCranePurpose
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "industrial" => Ok(Self::Industrial),
            "firesafe" => Ok(Self::FireSafe),
            "explosionsafe" => Ok(Self::ExplosionSafe),
            "marine" => Ok(Self::Marine),
            _ => Err(format!(
                "ExplosionFireSaveCranePurpose.from_str | Invalid ExplosionFireSaveCranePurpose: {}",
                s
            )
            .into()),
        }
    }
}
//
//
impl ToString for ExplosionFireSaveCranePurpose {
    ///
    /// Method translates from enuming structure `ExplosionFireSaveCranePurpose` into string
    fn to_string(&self) -> String {
        match self {
            ExplosionFireSaveCranePurpose::Industrial => "Industrial".to_string(),
            ExplosionFireSaveCranePurpose::FireSafe => "FireSafe".to_string(),
            ExplosionFireSaveCranePurpose::ExplosionSafe => "ExplosionSafe".to_string(),
            ExplosionFireSaveCranePurpose::Marine => "Marine".to_string(),
        }
    }
}
