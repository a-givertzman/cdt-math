use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Enum for structurizing cable winding methods
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CableWingingMethods {
    SinglelayerWingind,
    MultilayerWinding
}
//
//
impl FromStr for CableWingingMethods {
    type Err = StrErr;
    ///
    /// Method translates from string into enuming structure MechanismWorkType
    /// - 's' - value to translate
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "SinglelayerWingind" => Ok(Self::SinglelayerWingind),
            "MultilayerWinding" => Ok(Self::MultilayerWinding),
            _ => Err(format!("MechanismWorkType.from_str | Invalid MechanismWorkType: {}", s).into()),
        }
    }
}