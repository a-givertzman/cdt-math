use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов работы механизма подъема
#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]

pub enum MechanismWorkType {
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
}
//
impl FromStr for MechanismWorkType {
    ///
    /// Метод перевод из строки в тип перечисления MechanismWorkType
    type Err = StrErr;
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
            _ => Err(format!("MechanismWorkType.from_str | Invalid MechanismWorkType: {}", s).into()),
        }
    }
}
//
//
//
impl ToString for MechanismWorkType{
    ///
    /// Метод перевод из типа перечисления MechanismWorkType в строку
    fn to_string(&self) -> String {
        match self {
            MechanismWorkType::M1 => "M1".to_string(),
            MechanismWorkType::M2 => "M2".to_string(),
            MechanismWorkType::M3 => "M3".to_string(),
            MechanismWorkType::M4 => "M4".to_string(),
            MechanismWorkType::M5 => "M5".to_string(),
            MechanismWorkType::M6 => "M6".to_string(),
            MechanismWorkType::M7 => "M7".to_string(),
            MechanismWorkType::M8 => "M8".to_string(),
        }
    }
}