use std::str::FromStr;
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования степени уравновешенности каната
#[derive(Debug, Clone)]
pub enum HoistRopeBalanceDegree {
    Straightened,
    Unstraightened,
}
//
//
//
impl FromStr for HoistRopeBalanceDegree {
    ///
    /// Метод перевод из строки в тип перечисления DriverType
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "рихтованный" => Ok(Self::Straightened),
            "нерихтованный" => Ok(Self::Unstraightened),
            _ => Err(format!("DriverType.from_str | Invalid Cargo???DriverType: {}", s).into()),
        }
    }
}
//
//
//
impl ToString for HoistRopeBalanceDegree{
    fn to_string(&self) -> String {
        match self {
            HoistRopeBalanceDegree::Straightened => "рихтованный".to_string(),
            HoistRopeBalanceDegree::Unstraightened => "нерихтованный".to_string(),
        }
    }
}