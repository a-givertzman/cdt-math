use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования классов подъема
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LiftClass {
    Hc1,
    Hc2,
    Hc3,
    Hc4,    
}
//
//
//
impl FromStr for LiftClass {
    type Err = StrErr;
    ///
    /// Метод перевод из строки в тип перечисления LiftClass
     fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hc1" => Ok(Self::Hc1),
            "hc2" => Ok(Self::Hc2),
            "hc3" => Ok(Self::Hc3),
            "hc4" => Ok(Self::Hc4),
            _ => Err(format!("LiftClass.from_str | Invalid Cargo???LiftClass: {}", s).into()),
        }
    }
}