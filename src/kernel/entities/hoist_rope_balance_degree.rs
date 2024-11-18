use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования степени уравновешенности каната
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HoistRopeBalanceDegree {
    Straightened,
    Unstraightened,
}
//
impl FromStr for HoistRopeBalanceDegree {
    ///
    /// Метод перевод из строки в тип перечисления HoistRopeBalanceDegree
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "straightened" => Ok(Self::Straightened),
            "unstraightened" => Ok(Self::Unstraightened),
            _ => Err(format!("DriverType.from_str | Invalid Cargo???DriverType: {}", s).into()),
        }
    }
}
//
//
//
impl ToString for HoistRopeBalanceDegree{
    ///
    /// Метод перевод из типа перечисления HoistRopeBalanceDegree в строку 
    fn to_string(&self) -> String {
        match self {
            HoistRopeBalanceDegree::Straightened => "straightened".to_string(),
            HoistRopeBalanceDegree::Unstraightened => "unstraightened".to_string(),
        }
    }
}