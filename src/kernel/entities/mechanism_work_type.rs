use std::str::FromStr;
use crate::kernel::str_err::str_err::StrErr;
///
/// Перечисление для структуирования типов привода мех.под.
#[derive(Debug, Clone)]
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
//
//
impl FromStr for MechanismWorkType {
    ///
    /// Метод перевод из строки в тип перечисления DriverType
    type Err = StrErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "м1" => Ok(Self::M1),
            "м2" => Ok(Self::M2),
            "м3" => Ok(Self::M3),
            "м4" => Ok(Self::M4),
            "м5" => Ok(Self::M5),
            "м6" => Ok(Self::M6),
            "м7" => Ok(Self::M7),
            "м8" => Ok(Self::M8),
            _ => Err(format!("MechanismWorkType.from_str | Invalid MechanismWorkType: {}", s).into()),
        }
    }
}
//
//
//
impl ToString for MechanismWorkType{
    fn to_string(&self) -> String {
        self.to_string()
    }
}