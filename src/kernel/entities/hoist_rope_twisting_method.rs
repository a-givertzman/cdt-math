
use std::str::FromStr;
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования способ свивки каната
#[derive(Debug, Clone)]
pub enum HoistRopeTwistingMethod {
    Nontwisting,
    Twisting,
}
//
//
//
impl FromStr for HoistRopeTwistingMethod {
    ///
    /// Метод перевод из строки в тип перечисления HoistRopeTwistingMethod
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "нераскручивающийся" => Ok(Self::Nontwisting),
            "раскручивающийся" => Ok(Self::Twisting),
            _ => Err(format!("DriverType.from_str | Invalid Cargo???DriverType: {}", s).into()),
        }
    }
}
//
//
//
impl ToString for HoistRopeTwistingMethod{
    ///
    /// Метод перевод из типа перечисления HoistRopeTwistingMethod в строку
    fn to_string(&self) -> String {
        match self{
            HoistRopeTwistingMethod::Nontwisting => "нераскручивающийся".to_string(),
            HoistRopeTwistingMethod::Twisting => "раскручивающийся".to_string(),
        }
    }
}
