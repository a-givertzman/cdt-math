use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов каната
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HoistRopeType {
    Metal,
    Synthetic
}
//
impl FromStr for HoistRopeType {
    type Err = StrErr;
    ///
    /// Метод перевод из строки в тип перечисления HoistRopeType
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "metal" => Ok(Self::Metal),
            "synthetic" => Ok(Self::Synthetic),
            _ => Err(format!("DriverType.from_str | Invalid Cargo???DriverType: {}", s).into()),
        }
    }
}
//
//
//
impl ToString for HoistRopeType{
    ///
    /// Метод перевод из типа перечисления HoistRopeType в строку
    fn to_string(&self) -> String {
        match self{
            HoistRopeType::Metal => "металлический".to_string(),
            HoistRopeType::Synthetic => "синтетический".to_string(),
        }
    }
}