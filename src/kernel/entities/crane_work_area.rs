use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов рабочей местности крана
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CraneWorkArea {
    Normal,
    Aggressive,
    StrongAggresive,
}
//
//
//
impl FromStr for CraneWorkArea {
    type Err = StrErr;
    ///
    /// Метод перевод из строки в тип перечисления CraneWorkArea
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "normal" => Ok(Self::Normal),
            "aggressive" => Ok(Self::Aggressive),
            "strong aggresive" => Ok(Self::StrongAggresive),
            _ => Err(format!("CraneWorkArea.from_str | Invalid CraneWorkArea: {}", s).into()),
        }
    }
}